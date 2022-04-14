use std::io;

use crate::{
    crypto::{keypair::PublicKey, schnorr::Signature},
    impl_vec, net,
    util::serial::{
        deserialize, serialize, Decodable, Encodable, SerialDecodable, SerialEncodable, VarInt,
    },
    Result,
};

use super::{
    metadata::Metadata,
    participant::Participant,
    tx::Tx,
    util::{Timestamp, GENESIS_HASH_BYTES},
};

const SLED_BLOCK_TREE: &[u8] = b"_blocks";

/// This struct represents a tuple of the form (st, sl, txs).
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct Block {
    /// Previous block hash
    pub st: blake3::Hash,
    /// Slot uid, generated by the beacon
    pub sl: u64,
    /// Transaction hashes
    /// The actual transactions are in [`TxStore`]
    pub txs: Vec<blake3::Hash>,
}

impl Block {
    pub fn new(st: blake3::Hash, sl: u64, txs: Vec<blake3::Hash>) -> Block {
        Block { st, sl, txs }
    }
}

#[derive(Debug)]
pub struct BlockStore(sled::Tree);

impl BlockStore {
    /// Opens a new or existing blockstore tree given a sled database.
    pub fn new(db: &sled::Db) -> Result<Self> {
        let tree = db.open_tree(SLED_BLOCK_TREE)?;
        let store = Self(tree);
        if store.0.is_empty() {
            // Genesis block is generated.
            let hash = blake3::Hash::from(GENESIS_HASH_BYTES);
            let genesis_block = Block::new(hash, 0, vec![]);
            store.insert(&genesis_block)?;
        }

        Ok(store)
    }

    /// Insert a block into the blockstore.
    /// The block is hashed with blake3 and this blockhash is used as
    /// the key, where value is the serialized block itself.
    pub fn insert(&self, block: &Block) -> Result<blake3::Hash> {
        let serialized = serialize(block);
        let blockhash = blake3::hash(&serialized);
        self.0.insert(blockhash.as_bytes(), serialized)?;

        Ok(blockhash)
    }

    /// Retrieve all blocks.
    /// Be carefull as this will try to load everything in memory.
    pub fn get_all(&self) -> Result<Vec<Option<(blake3::Hash, Block)>>> {
        let mut blocks = Vec::new();
        let mut iterator = self.0.into_iter().enumerate();
        while let Some((_, r)) = iterator.next() {
            let (k, v) = r.unwrap();
            let hash_bytes: [u8; 32] = k.as_ref().try_into().unwrap();
            let block = deserialize(&v)?;
            blocks.push(Some((hash_bytes.into(), block)));
        }
        Ok(blocks)
    }
}

/// This struct represents a Block proposal, used for consensus.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct BlockProposal {
    /// leader public key
    pub public_key: PublicKey,
    /// signed block
    pub signature: Signature,
    /// leader id
    pub id: u64,
    /// Previous block hash
    pub st: blake3::Hash,
    /// Slot uid, generated by the beacon
    pub sl: u64,
    /// Transactions payload
    pub txs: Vec<Tx>,
    /// Additional proposal information
    pub metadata: Metadata,
}

impl BlockProposal {
    pub fn new(
        public_key: PublicKey,
        signature: Signature,
        id: u64,
        st: blake3::Hash,
        sl: u64,
        txs: Vec<Tx>,
        timestamp: Timestamp,
        proof: String,
        r: String,
        s: String,
        participants: Vec<Participant>,
    ) -> BlockProposal {
        BlockProposal {
            public_key,
            signature,
            id,
            st,
            sl,
            txs,
            metadata: Metadata::new(timestamp, proof, r, s, participants),
        }
    }
}

impl PartialEq for BlockProposal {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key &&
            self.signature == other.signature &&
            self.id == other.id &&
            self.st == other.st &&
            self.sl == other.sl &&
            self.txs == other.txs
    }
}

impl net::Message for BlockProposal {
    fn name() -> &'static str {
        "proposal"
    }
}

impl_vec!(BlockProposal);
