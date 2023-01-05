use fvm_integration_tests::tester::{Account, Tester};
use fvm_integration_tests::dummy::DummyExterns;
use fvm_integration_tests::bundle;
use fvm_ipld_encoding::{strict_bytes, tuple::*};
use fvm_shared::state::StateTreeVersion;
use fvm_shared::version::NetworkVersion;
use fvm_ipld_blockstore::MemoryBlockstore;
use anyhow::{Context, Result};
use fvm_ipld_encoding::CborStore;
use cid::Cid;
use fvm_ipld_blockstore::Blockstore;
use fvm_shared::ActorID;
use fvm::state_tree::{ActorState, StateTree};
use multihash::Code;
use std::env;
use std::str::FromStr;
use fvm_shared::message::Message;
use fvm::executor::{ApplyKind, Executor};
use fil_actor_eam::Return;
use fvm_ipld_encoding::RawBytes;
use fil_actors_runtime::{EAM_ACTOR_ADDR};
use fvm_shared::address::Address;

const WASM_COMPILED_PATH: &str =
"../build/SimpleCoin.bin";


#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct Create2Params {
    #[serde(with = "strict_bytes")]
    pub initcode: Vec<u8>,
    #[serde(with = "strict_bytes")]
    pub salt: [u8; 32],
}

fn main() {
    println!("Testing solidity API");

    let bs = MemoryBlockstore::default();
    let actors = std::fs::read("./builtin-actors-devnet-wasm.car").expect("Unable to read actor devnet file file");
    let bundle_root = bundle::import_bundle(&bs, &actors).unwrap();

    let mut tester =
        Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap();

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // INIT STORAGE POWER SINGLETON
    let state_tree = tester.state_tree.as_mut().unwrap();
    let storagemarket_code_cid = Cid::from_str("bafk2bzacedreyxpkabexd4lrmcjzenfkjyfrbzns2o3p7cokfm6b7iendal2o").unwrap();

    set_storagepower_actor(state_tree, storagemarket_code_cid);

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();

    let executor = tester.executor.as_mut().unwrap();

    // Try to call "constructor"
    println!("Try to call constructor on storage power actor");

    let message = Message {
        from: Address::new_id(0),
        to: Address::new_id(4),
        gas_limit: 1000000000,
        method_num: 2,
        ..Message::default()
    };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();

    dbg!(&res);

    assert_eq!(res.msg_receipt.exit_code.value(), 0);

    println!("Calling init actor (EVM)");

    let wasm_path = env::current_dir()
        .unwrap()
        .join(WASM_COMPILED_PATH)
        .canonicalize()
        .unwrap();
    let evm_bin = std::fs::read(wasm_path).expect("Unable to read file");

    let constructor_params = Create2Params {
        initcode: evm_bin,
        salt: [0; 32],
    };

    let message = Message {
        from: sender[0].1,
        to: EAM_ACTOR_ADDR,
        gas_limit: 1000000000,
        method_num: 3,
        params: RawBytes::serialize(constructor_params).unwrap(),
        ..Message::default()
    };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();

    dbg!(&res);

    assert_eq!(res.msg_receipt.exit_code.value(), 0);

    let exec_return : Return = RawBytes::deserialize(&res.msg_receipt.return_data).unwrap();
}


pub fn set_storagepower_actor(
    state_tree: &mut StateTree<impl Blockstore>,
    storagepower_code_cid: Cid,
) -> Result<()> {
    const STORAGE_POWER_ACTOR: ActorID = 4;

    let storagepower_state_cid = state_tree
        .store()
        .put_cbor(&[(); 0], Code::Blake2b256)
        .context("storagepower actor".to_owned())?;

    let storagepower_actor_state = ActorState {
        code: storagepower_code_cid,
        state: storagepower_state_cid,
        sequence: 0,
        balance: Default::default(),
        delegated_address: None,
    };

    state_tree
        .set_actor(STORAGE_POWER_ACTOR, storagepower_actor_state)
        .map_err(anyhow::Error::from)
        .context("storagepower actor".to_owned())
}