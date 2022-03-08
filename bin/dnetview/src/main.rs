use darkfi::{
    error::{Error, Result},
    rpc::{jsonrpc, jsonrpc::JsonResult},
    util::{
        async_util,
        cli::{log_config, spawn_config, Config},
        join_config_path,
    },
};

use async_std::sync::Arc;
use easy_parallel::Parallel;
use log::{debug, info};
use serde_json::{json, Value};
use simplelog::*;
use smol::Executor;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io,
    io::Read,
    path::PathBuf,
};
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};
use url::Url;

use dnetview::{
    config::{DnvConfig, CONFIG_FILE_CONTENTS},
    model::{Connection, IdList, InfoList, NodeInfo},
    options::ProgramOptions,
    ui,
    view::{IdListView, InfoListView},
    Model, View,
};

struct Map {
    url: Url,
}

impl Map {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    async fn request(&self, r: jsonrpc::JsonRequest) -> Result<Value> {
        let reply: JsonResult = match jsonrpc::send_request(&self.url, json!(r), None).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        match reply {
            JsonResult::Resp(r) => {
                debug!(target: "RPC", "<-- {}", serde_json::to_string(&r)?);
                Ok(r.result)
            }

            JsonResult::Err(e) => {
                debug!(target: "RPC", "<-- {}", serde_json::to_string(&e)?);
                Err(Error::JsonRpcError(e.error.message.to_string()))
            }

            JsonResult::Notif(n) => {
                debug!(target: "RPC", "<-- {}", serde_json::to_string(&n)?);
                Err(Error::JsonRpcError("Unexpected reply".to_string()))
            }
        }
    }

    // --> {"jsonrpc": "2.0", "method": "ping", "params": [], "id": 42}
    // <-- {"jsonrpc": "2.0", "result": "pong", "id": 42}
    async fn _ping(&self) -> Result<Value> {
        let req = jsonrpc::request(json!("ping"), json!([]));
        Ok(self.request(req).await?)
    }

    //--> {"jsonrpc": "2.0", "method": "poll", "params": [], "id": 42}
    // <-- {"jsonrpc": "2.0", "result": {"nodeID": [], "nodeinfo" [], "id": 42}
    async fn get_info(&self) -> Result<Value> {
        let req = jsonrpc::request(json!("get_info"), json!([]));
        Ok(self.request(req).await?)
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let options = ProgramOptions::load()?;

    let verbosity_level = options.app.occurrences_of("verbose");

    let (lvl, cfg) = log_config(verbosity_level)?;

    let file = File::create(&*options.log_path).unwrap();
    WriteLogger::init(lvl, cfg, file)?;
    info!("Log level: {}", lvl);

    let config_path = join_config_path(&PathBuf::from("dnetview_config.toml"))?;

    spawn_config(&config_path, CONFIG_FILE_CONTENTS)?;

    let config = Config::<DnvConfig>::load(config_path)?;

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let info_list = InfoList::new();
    let ids = HashSet::new();
    let id_list = IdList::new(ids);

    let model = Arc::new(Model::new(id_list, info_list));

    let nthreads = num_cpus::get();
    let (signal, shutdown) = async_channel::unbounded::<()>();

    let ex = Arc::new(Executor::new());
    let ex2 = ex.clone();

    let (_, result) = Parallel::new()
        .each(0..nthreads, |_| smol::future::block_on(ex.run(shutdown.recv())))
        .finish(|| {
            smol::future::block_on(async move {
                run_rpc(&config, ex2.clone(), model.clone()).await?;
                render(&mut terminal, model.clone(), config).await?;
                drop(signal);
                Ok::<(), darkfi::Error>(())
            })
        });

    result
}

async fn run_rpc(config: &DnvConfig, ex: Arc<Executor<'_>>, model: Arc<Model>) -> Result<()> {
    for node in config.nodes.clone() {
        let client = Map::new(Url::parse(&node.node_id)?);
        ex.spawn(poll(client, model.clone())).detach();
    }
    Ok(())
}

async fn poll(client: Map, model: Arc<Model>) -> Result<()> {
    debug!("Attemping to poll: {}", client.url);
    loop {
        let reply = client.get_info().await?;
        debug!("{:?}", reply);

        if reply.as_object().is_some() && !reply.as_object().unwrap().is_empty() {
            let external_addr = reply.as_object().unwrap().get("external_addr");
            let session_inbound = reply.as_object().unwrap().get("session_inbound");
            let si_key =
                session_inbound.unwrap().as_object().unwrap().get("key").unwrap().as_u64().unwrap();

            let session_manual = reply.as_object().unwrap().get("session_manual");
            let sm_key =
                session_manual.unwrap().as_object().unwrap().get("key").unwrap().as_u64().unwrap();

            let session_outbound = reply.as_object().unwrap().get("session_outbound");
            let so_key =
                session_manual.unwrap().as_object().unwrap().get("key").unwrap().as_u64().unwrap();

            let channel_state = reply.as_object().unwrap().get("state").unwrap().as_str().unwrap();
            let slots = reply.as_object().unwrap().get("slots");

            let session_in = Connection::new(si_key.to_string(), channel_state.to_string());
            let session_man = Connection::new(sm_key.to_string(), channel_state.to_string());
            let session_out = Connection::new(so_key.to_string(), channel_state.to_string());

            let mut outconnects = Vec::new();
            let mut inconnects = Vec::new();
            let mut manconnects = Vec::new();

            outconnects.push(session_out);
            inconnects.push(session_in);
            manconnects.push(session_man);

            let infos =
                NodeInfo { outbound: outconnects, manual: manconnects, inbound: inconnects };

            let mut node_info = HashMap::new();
            // TODO: fix this. this key should be global identifier for each connection
            // right now we are using si_key, which is the inbound session key.
            node_info.insert(si_key, infos);

            for (si_key, value) in node_info.clone() {
                model.id_list.node_id.lock().await.insert(si_key.to_string().clone());
                model.info_list.infos.lock().await.insert(si_key.to_string(), value);
            }
        } else {
            // TODO: error handling
            debug!("Reply is empty");
        }
        async_util::sleep(2).await;
    }
}

async fn render<B: Backend>(
    terminal: &mut Terminal<B>,
    model: Arc<Model>,
    config: DnvConfig,
) -> io::Result<()> {
    let mut asi = async_stdin();

    terminal.clear()?;

    let id_list = IdListView::new(HashSet::new());
    let info_list = InfoListView::new(HashMap::new());
    let mut view = View::new(id_list.clone(), info_list.clone());

    view.id_list.state.select(Some(0));
    view.info_list.index = 0;

    loop {
        view.update(model.info_list.infos.lock().await.clone());
        terminal.draw(|f| {
            ui::ui(f, view.clone(), &config);
        })?;
        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Char('j') => {
                    view.id_list.next();
                    view.info_list.next().await;
                }
                Key::Char('k') => {
                    view.id_list.previous();
                    view.info_list.previous().await;
                }
                _ => (),
            }
        }
    }
}
