use incrementalmerkletree::{bridgetree::BridgeTree, Frontier, Tree};

use halo2_gadgets::primitives::{
    poseidon,
    poseidon::{ConstantLength, P128Pow5T3},
};

use halo2_proofs::{
    dev::MockProver,
};

use rand::{thread_rng, Rng};

use pasta_curves::{pallas, Fp};

use darkfi::{
    zk:: {
        circuit::lead_contract::{LeadContract},
    },
    crypto::{
        merkle_node::MerkleNode,
        keypair::{Keypair, PublicKey, SecretKey},
        types::*,
        constants::{
            NullifierK, OrchardFixedBases, OrchardFixedBasesFull, ValueCommitV, MERKLE_DEPTH_ORCHARD,
        },
        nullifier::Nullifier,
        proof::{Proof, ProvingKey, VerifyingKey},
        util::{mod_r_p, pedersen_commitment_scalar, pedersen_commitment_u64},
    },
};

use pasta_curves::group::Curve;
use pasta_curves::arithmetic::CurveAffine;
//use halo2_proofs::arithmetic::CurveAffine;

#[derive(Debug,Default,Clone,Copy)]
pub struct Coin
{
    value : Option<pallas::Base>, //stake
    cm : Option<pallas::Point>,
    cm2 : Option<pallas::Point>,
    cm_blind : Option<pallas::Base>,
    sl : Option<pallas::Base>, //slot id
    tau : Option<pallas::Base>,
    nonce : Option<pallas::Base>,
    sn : Option<pallas::Point>, // coin's serial number
    //sk : Option<SecretKey>,
    pk : Option<pallas::Point>,
    root_cm : Option<pallas::Scalar>,
    root_sk : Option<pallas::Scalar>,
    path: Option<[MerkleNode; MERKLE_DEPTH_ORCHARD]>,
    path_sk: Option<[MerkleNode; MERKLE_DEPTH_ORCHARD]>,
    opening1 : Option<pallas::Base>,
    opening2 : Option<pallas::Base>,
}

fn main()
{
    let k = 13;
    //
    //TODO calculate commitment here
    //this is the commitment of the first coin
    //TODO construct a tree of multiple coins
    const LEN : usize = 10;
    let mut rng = thread_rng();
    let mut sks : Vec<u64> = vec![];
    let mut root_sks : Vec<MerkleNode> = vec![];
    let mut path_sks : Vec<[MerkleNode;MERKLE_DEPTH_ORCHARD]> = vec![];
    let mut tree  =  BridgeTree::<MerkleNode, 32>::new(LEN);
    for i in 0..LEN {
        let tmp : u64 = rng.gen();
        let mut sk : u64 = tmp;
        sks.push(sk.clone());
        let node = MerkleNode(pallas::Base::from(sk));
        tree.append(&node.clone());
        let (leaf_pos, path) = tree.authentication_path(&node).unwrap();
        root_sks.push(tree.root().clone());
        path_sks.push(path.as_slice().try_into().unwrap());
    }
    let mut seeds : Vec<u64> = vec![];
    for i in 0..LEN {
        let rho : u64 = rng.gen();
        seeds.push(rho.clone());
    }
    //
    let yu64 : u64 = rng.gen();
    let rhou64 : u64 = rng.gen();
    let mau_y : pallas::Scalar = pallas::Scalar::from(yu64);
    let mau_rho : pallas::Scalar = pallas::Scalar::from(rhou64);

    //
    let mut coins : Vec<Coin> = vec![];

    //
    let mut tree_cm = BridgeTree::<MerkleNode, 32>::new(LEN);
    let zerou64 : u64 = 0;
    for i in 0..LEN {
        let c_v = pallas::Base::from(u64::try_from(i*2).unwrap());
        //random sampling of the same size of prf,
        //pseudo random sampling that is the size of pederson commitment
        let c_sk : u64 = sks[i];
        let iu64 : u64 = u64::try_from(i).unwrap();
        let c_sl  = pallas::Base::from(iu64);

        let c_tau  = pallas::Base::from(u64::try_from(i).unwrap()); // let's assume it's sl for simplicity
        let c_root_sk : MerkleNode  = root_sks[i];
        // =========================
        //TODO 512 secret-key/public-key to cop with pallas curves
        // =========================
        //note! sk is used in MerkleNode takes pallas::Base as input
        //while the pallas::base is 512, the SecretKey is  of size 256, a larger keyring is needed
        //TODO what is the endianess of this keyring
        //let sk_bits = vec![];
        //sk_bits.append(&mut c_sk.to_le_bytes().to_vec());
        //sk_bits.append(&mut zerou64.to_le_bytes().to_vec());
        //sk_bits.append(&mut zerou64.to_le_bytes().to_vec());
        //sk_bits.append(&mut zerou64.to_le_bytes().to_vec());
        //let c_pk = PublicKey::from_secret(SecretKey::from_bytes(sk_bits.as_slice().try_into().unwrap()).unwrap());
        let c_pk = pedersen_commitment_scalar(mod_r_p(c_tau), mod_r_p(c_root_sk.inner()));
        //
        // TODO (fix) no use random value for the secret key as random pallas base
        // =======================


        let c_seed  = pallas::Base::from(seeds[i]);
        let c_sn  = pedersen_commitment_scalar(mod_r_p(c_seed), mod_r_p(c_root_sk.inner()));
        let c_pk_pt = c_pk.to_affine().coordinates().unwrap();
        let c_cm_message = [*c_pk_pt.x(), *c_pk_pt.y(), c_v.clone(), c_seed.clone()];
        let c_cm_v = poseidon::Hash::<_,P128Pow5T3, ConstantLength<4>, 3, 2>::init().hash(c_cm_message);
        let c_cm1_blind = pallas::Base::from(0); //tmp val
        let c_cm2_blind = pallas::Base::from(0); //tmp val
        let c_cm : pallas::Point  = pedersen_commitment_scalar(mod_r_p(c_cm_v), mod_r_p(c_cm1_blind));
        //TODO (fix) which affine coefficient point to be used a/b ?
        let c_cm_node = MerkleNode(*c_cm.to_affine().coordinates().unwrap().x()); //CurveAffine::a()
        tree_cm.append(&c_cm_node.clone());
        let (leaf_pos, c_cm_path) = tree_cm.authentication_path(&c_cm_node).unwrap();
        let c_root_cm = tree_cm.root();
        // lead coin commitment
        //TODO this c_v can be
        let c_seed2 = pedersen_commitment_scalar(mod_r_p(c_seed), mod_r_p(c_root_sk.inner()));
        let c_seed2_pt = c_seed2.to_affine().coordinates().unwrap();
        let lead_coin_msg = [*c_pk_pt.x(), *c_pk_pt.y(), c_v, *c_seed2_pt.x(), *c_seed2_pt.y()];
        let lead_coin_msg_hash = poseidon::Hash::<_,P128Pow5T3, ConstantLength<5>, 3, 2>::init().hash(lead_coin_msg);
        let c_cm2 = pedersen_commitment_scalar(mod_r_p(lead_coin_msg_hash), mod_r_p(c_cm2_blind));
        let c_root_sk = root_sks[i];
        let c_path_sk = path_sks[i];
        let coin  = Coin {
            value: Some(c_v),
            cm: Some(c_cm),
            cm2: Some(c_cm2),
            cm_blind: Some(c_cm1_blind),
            sl: Some(c_sl),
            tau: Some(c_tau),
            nonce: Some(c_seed),
            sn:  Some(c_sn),
            //sk: Some(c_sk),
            pk: Some(c_pk),
            root_cm: Some(mod_r_p(c_root_cm.inner())),
            root_sk: Some(mod_r_p(c_root_sk.inner())),
            path: Some(c_cm_path.as_slice().try_into().unwrap()),
            path_sk: Some(c_path_sk),
            opening1: Some(c_cm1_blind),
            opening2: Some(c_cm2_blind),
        };
        coins.push(coin);
    }

    // ================
    // public inputs
    // ================
    let coin_idx  = 0;
    let coin = coins[coin_idx];
    let c0 = pedersen_commitment_scalar(mod_r_p(coin.nonce.unwrap()), coin.root_cm.unwrap())
        .to_affine()
        .coordinates()
        .unwrap();
    let c1 = pedersen_commitment_scalar(mod_r_p(coin.tau.unwrap()), coin.root_cm.unwrap())
        .to_affine()
        .coordinates()
        .unwrap();
    //TODO root_cm need to be converted to Fp
    let c2 = pedersen_commitment_scalar(mod_r_p(coin.nonce.unwrap()), coin.root_cm.unwrap())
        .to_affine()
        .coordinates()
        .unwrap();
   //
    let c3 = coin.cm.unwrap().to_affine().coordinates().unwrap();
    let c4 = coin.cm2.unwrap().to_affine().coordinates().unwrap();

    //TODO (fix) public key is a commitemnet of the root of secret key, and the timestamp
    let c7 = coin.pk.unwrap().to_affine().coordinates().unwrap();
    let c8 = coin.sn.unwrap().to_affine().coordinates().unwrap();
    //TODO (fix) this need to be replaced by computed final path as pallas::Base
    let c5 = coin.path.unwrap();
    let c6 = pallas::Base::from(0);
    // ===============

    let path_sk = path_sks[coin_idx];
    let contract = LeadContract {
        path: coin.path,
        root_sk: coin.root_sk,
        path_sk: Some(path_sk),
        coin_timestamp: coin.tau, //
        coin_nonce: coin.nonce,
        coin_opening_1: Some(mod_r_p(coin.opening1.unwrap())),
        value: coin.value,
        coin_opening_2: Some(mod_r_p(coin.opening2.unwrap())),
        cm_c1_x: Some(*c3.x()),
        cm_c1_y: Some(*c3.y()),
        cm_c2_x: Some(*c4.x()),
        cm_c2_y: Some(*c4.y()),
        cm_pos : Some(u32::try_from(coin_idx).unwrap()),
        //sn_c1: Some(coin.sn.unwrap()),
        slot: Some(coin.sl.unwrap()),
        mau_rho: Some(mau_rho.clone()),
        mau_y: Some(mau_y.clone()),
        root_cm: Some(coin.root_cm.unwrap()),
    };

    let mut public_inputs = vec![*c0.x(), *c0.y(),
                                 *c1.x(), *c1.y(),
                                 *c2.x(), *c2.y(),
                                 *c7.x(), *c7.y(),
                                 *c8.x(), *c8.y(),
                                 *c3.x(), *c3.y(),
                                 *c4.x(), *c4.y(),
                                 c5[31].inner(), //TODO (res) how the path is structured assumed root is last node in the path.
                                 c6,
    ];

    let mut vec_inputs = vec![public_inputs];
    //TODO
    let prover = MockProver::run(k, &contract, vec_inputs).unwrap();
    assert_eq!(prover.verify(), Ok(()));
}
