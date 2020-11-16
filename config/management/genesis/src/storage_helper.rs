// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

// FIXME: (gnazario) storage helper doesn't belong in the genesis tool, but it's attached to it right now

use crate::command::Command;
use consensus_types::safety_data::SafetyData;
use libra_crypto::{Uniform, ValidCryptoMaterialStringExt, ed25519::{Ed25519PrivateKey, Ed25519PublicKey}};
use libra_global_constants::{
    CONSENSUS_KEY, EXECUTION_KEY, FULLNODE_NETWORK_KEY, LIBRA_ROOT_KEY, OPERATOR_KEY, OWNER_KEY,
    SAFETY_DATA, TREASURY_COMPLIANCE_KEY, VALIDATOR_NETWORK_KEY, WAYPOINT,
};
use libra_management::{error::Error, secure_backend::DISK};
use libra_network_address::NetworkAddress;
use libra_secure_storage::{CryptoStorage, KVStorage, NamespacedStorage, OnDiskStorage, Storage};
use libra_types::{chain_id::ChainId, transaction::Transaction, waypoint::Waypoint};
use vm_genesis::GenesisMiningProof;
use std::{fs::File, path::{Path, PathBuf}};
use structopt::StructOpt;

//////// 0L ////////
use miner::node_keys::KeyScheme;

pub struct StorageHelper {
    temppath: libra_temppath::TempPath,
}

impl StorageHelper {
    pub fn new() -> Self {
        let temppath = libra_temppath::TempPath::new();
        temppath.create_as_file().unwrap();
        File::create(temppath.path()).unwrap();
        Self { temppath }
    }

    //////// 0L ////////
    pub fn new_with_path(path: PathBuf) -> Self {
        let path = libra_temppath::TempPath::new_with_dir(path);
        path.create_as_file().expect("Failed on create_as_file");
        File::create(path.path()).expect("Could not create file");
        Self { temppath: path }
    }

    ///////// 0L  /////////
    pub fn get_with_path(path: PathBuf) -> Self {
        let path = libra_temppath::TempPath::new_with_dir(path);
        // path.create_as_file().expect("Failed on create_as_file");
        // File::create(path.path()).expect("Could not create file");
        Self { temppath: path }
    }

    ///////// 0L  /////////
    pub fn initialize_with_mnemonic_swarm(&self, namespace: String, mnemonic: String) {
        let keys = KeyScheme::new_from_mnemonic(mnemonic);
        let mut storage = self.storage(namespace.clone());
        // let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([5; 32]);
        let dummy_root = Ed25519PrivateKey::from_encoded_string("8108aedfacf5cf1d73c67b6936397ba5fa72817f1b5aab94658238ddcdc08010").unwrap();

        storage
            .import_private_key(LIBRA_ROOT_KEY, dummy_root.clone())
            .unwrap();
        // let libra_root_key = storage_owner.export_private_key(LIBRA_ROOT_KEY).unwrap();
        storage
            .import_private_key(TREASURY_COMPLIANCE_KEY, dummy_root)
            .unwrap();
        storage
            .import_private_key(OWNER_KEY, keys.child_0_owner.get_private_key())
            .unwrap();
        storage
            .import_private_key(OPERATOR_KEY, keys.child_1_operator.get_private_key())
            .unwrap();
        storage
            .import_private_key(VALIDATOR_NETWORK_KEY, keys.child_2_val_network.get_private_key())
            .unwrap();
        storage
            .import_private_key(FULLNODE_NETWORK_KEY, keys.child_3_fullnode_network.get_private_key())
            .unwrap();
        storage
            .import_private_key(CONSENSUS_KEY, keys.child_4_consensus.get_private_key())
            .unwrap();
        storage
            .import_private_key(EXECUTION_KEY, keys.child_5_executor.get_private_key())
            .unwrap();
        storage
            .set(SAFETY_DATA, SafetyData::new(0, 0, 0, None))
            .unwrap();
        storage.set(WAYPOINT, Waypoint::default()).unwrap();
        
        let mut encryptor = libra_network_address_encryption::Encryptor::new(storage);
        encryptor.initialize().unwrap();

        // TODO: Use EncNetworkAddress instead of TEST_SHARED
        encryptor
            .add_key(
            libra_network_address::encrypted::TEST_SHARED_VAL_NETADDR_KEY_VERSION,
            libra_network_address::encrypted::TEST_SHARED_VAL_NETADDR_KEY,
            )
            .unwrap();
    }

    ///////// 0L  /////////
    pub fn initialize_with_mnemonic(&self, namespace: String, mnemonic: String) {
        let keys = KeyScheme::new_from_mnemonic(mnemonic);
        let mut storage_root = self.storage("root".to_owned());
        let mut storage_owner = self.storage(namespace.clone());
        let mut storage_oper = self.storage(namespace.clone() + "-oper");

        // let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([5; 32]);
        let dummy_root = Ed25519PrivateKey::from_encoded_string("8108aedfacf5cf1d73c67b6936397ba5fa72817f1b5aab94658238ddcdc08010").unwrap();

        storage_root
            .import_private_key(LIBRA_ROOT_KEY, dummy_root.clone())
            .unwrap();
        // let libra_root_key = storage_owner.export_private_key(LIBRA_ROOT_KEY).unwrap();
        storage_root
            .import_private_key(TREASURY_COMPLIANCE_KEY, dummy_root)
            .unwrap();
        storage_owner
            .import_private_key(OWNER_KEY, keys.child_0_owner.get_private_key())
            .unwrap();
        storage_oper
            .import_private_key(OPERATOR_KEY, keys.child_1_operator.get_private_key())
            .unwrap();
        storage_oper
            .import_private_key(VALIDATOR_NETWORK_KEY, keys.child_2_val_network.get_private_key())
            .unwrap();
        storage_oper
            .import_private_key(FULLNODE_NETWORK_KEY, keys.child_3_fullnode_network.get_private_key())
            .unwrap();
        storage_oper
            .import_private_key(CONSENSUS_KEY, keys.child_4_consensus.get_private_key())
            .unwrap();
        storage_oper
            .import_private_key(EXECUTION_KEY, keys.child_5_executor.get_private_key())
            .unwrap();
        storage_oper
            .set(SAFETY_DATA, SafetyData::new(0, 0, 0, None))
            .unwrap();
        storage_oper.set(WAYPOINT, Waypoint::default()).unwrap();
        
        let mut encryptor = libra_network_address_encryption::Encryptor::new(storage_oper);
        encryptor.initialize().unwrap();

        // TODO: Use EncNetworkAddress instead of TEST_SHARED
        encryptor
            .add_key(
            libra_network_address::encrypted::TEST_SHARED_VAL_NETADDR_KEY_VERSION,
            libra_network_address::encrypted::TEST_SHARED_VAL_NETADDR_KEY,
            )
            .unwrap();
    }

    pub fn storage(&self, namespace: String) -> Storage {
        let storage = OnDiskStorage::new(self.temppath.path().to_path_buf());
        Storage::from(NamespacedStorage::new(Storage::from(storage), namespace))
    }

    pub fn path(&self) -> &Path {
        self.temppath.path()
    }

    pub fn path_string(&self) -> &str {
        self.temppath.path().to_str().unwrap()
    }

    // pub fn initialize_by_idx(&self, namespace: String, idx: usize) {
    //     let partial_seed = lcs::to_bytes(&idx).unwrap();
    //     let mut seed = [0u8; 32];
    //     let data_to_copy = 32 - std::cmp::min(32, partial_seed.len());
    //     seed[data_to_copy..].copy_from_slice(partial_seed.as_slice());
    //     self.initialize(namespace, seed);
    // }
    
    

    // 0L: change, initialize the 0th account with a fixture mnemonic "Alice". So we can test miner and other APIs.
    pub fn initialize_by_idx(&self, namespace: String, idx: usize) {
        let mnem_alice = "talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse".to_string();
        let partial_seed = lcs::to_bytes(&idx).unwrap();
        let mut seed = [0u8; 32];
        let data_to_copy = 32 - std::cmp::min(32, partial_seed.len());
        seed[data_to_copy..].copy_from_slice(partial_seed.as_slice());
        // idx 0 is for libra account in swarm tests.
        // idx 1  is for the first node OWNER, set a fixed mnemonic to derive keys for this one so we can simulate miner workflow.
        if idx == 1 {
            self.initialize_with_mnemonic_swarm(namespace, mnem_alice);
        } else {
            self.initialize(namespace, seed);
        }
    }

    pub fn initialize(&self, namespace: String, seed: [u8; 32]) {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
        let mut storage = self.storage(namespace);

        // Initialize all keys in storage
        storage
            .import_private_key(LIBRA_ROOT_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();
        // TODO(davidiw) use distinct keys in tests for treasury and libra root keys
        let libra_root_key = storage.export_private_key(LIBRA_ROOT_KEY).unwrap();
        storage
            .import_private_key(TREASURY_COMPLIANCE_KEY, libra_root_key)
            .unwrap();
        storage
            .import_private_key(CONSENSUS_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();
        storage
            .import_private_key(EXECUTION_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();
        storage
            .import_private_key(FULLNODE_NETWORK_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();
        storage
            .import_private_key(OWNER_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();
        storage
            .import_private_key(OPERATOR_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();
        storage
            .import_private_key(VALIDATOR_NETWORK_KEY, Ed25519PrivateKey::generate(&mut rng))
            .unwrap();

        // Initialize all other data in storage
        storage
            .set(SAFETY_DATA, SafetyData::new(0, 0, 0, None))
            .unwrap();
        storage.set(WAYPOINT, Waypoint::default()).unwrap();
        let mut encryptor = libra_network_address_encryption::Encryptor::new(storage);
        encryptor.initialize().unwrap();
        encryptor
            .add_key(
                libra_network_address::encrypted::TEST_SHARED_VAL_NETADDR_KEY_VERSION,
                libra_network_address::encrypted::TEST_SHARED_VAL_NETADDR_KEY,
            )
            .unwrap();
    }

    ///////// 0L /////////
    pub fn swarm_pow_helper(&self, namespace: String){
        let mut storage = self.storage(namespace);
        let default_proof = GenesisMiningProof::default();
        storage.set(libra_global_constants::PROOF_OF_WORK_PREIMAGE, default_proof.preimage).unwrap();
        storage.set(libra_global_constants::PROOF_OF_WORK_PROOF, default_proof.proof).unwrap();
    }

    pub fn create_waypoint(&self, chain_id: ChainId) -> Result<Waypoint, Error> {
        let args = format!(
            "
                libra-genesis-tool
                create-waypoint
                --chain-id {chain_id}
                --shared-backend backend={backend};\
                    path={path}
            ",
            chain_id = chain_id,
            backend = DISK,
            path = self.path_string(),
        );

        let command = Command::from_iter(args.split_whitespace());
        command.create_waypoint()
    }

    ///////// 0L  /////////
    pub fn create_waypoint_gh(&self, chain_id: ChainId, remote: &str ) -> Result<Waypoint, Error> {
        let args = format!(
            "
                libra-genesis-tool
                create-waypoint
                --chain-id {chain_id}
                --shared-backend {remote}
            ",
            chain_id = chain_id,
            remote = remote,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.create_waypoint()
    }

    pub fn insert_waypoint(&self, validator_ns: &str, waypoint: Waypoint) -> Result<(), Error> {
        let args = format!(
            "
                libra-genesis-tool
                insert-waypoint
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={validator_ns}
                --waypoint {waypoint}
            ",
            backend = DISK,
            path = self.path_string(),
            validator_ns = validator_ns,
            waypoint = waypoint,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.insert_waypoint()
    }

    pub fn genesis(&self, chain_id: ChainId, genesis_path: &Path) -> Result<Transaction, Error> {
        let args = format!(
            "
                libra-genesis-tool
                genesis
                --chain-id {chain_id}
                --shared-backend backend={backend};\
                    path={path}
                --path {genesis_path}
            ",
            chain_id = chain_id,
            backend = DISK,
            path = self.path_string(),
            genesis_path = genesis_path.to_str().expect("Unable to parse genesis_path"),
        );

        let command = Command::from_iter(args.split_whitespace());
        command.genesis()
    }

    pub fn genesis_gh(&self, chain_id: ChainId, remote: &str, genesis_path: &PathBuf) -> Result<Transaction, Error> {
        let args = format!(
            "
                libra-genesis-tool
                genesis
                --chain-id {chain_id}
                --shared-backend {remote} 
                --path {genesis_path}
            ",
            chain_id = chain_id,
            remote = remote,
            genesis_path = genesis_path.to_str().expect("Unable to parse genesis_path"),
        );

        let command = Command::from_iter(args.split_whitespace());
        command.genesis()
    }

    pub fn libra_root_key(
        &self,
        validator_ns: &str,
        shared_ns: &str,
    ) -> Result<Ed25519PublicKey, Error> {
        let args = format!(
            "
                libra-genesis-tool
                libra-root-key
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={validator_ns}
                --shared-backend backend={backend};\
                    path={path};\
                    namespace={shared_ns}
            ",
            backend = DISK,
            path = self.path_string(),
            validator_ns = validator_ns,
            shared_ns = shared_ns,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.libra_root_key()
    }

    pub fn operator_key(
        &self,
        validator_ns: &str,
        shared_ns: &str,
    ) -> Result<Ed25519PublicKey, Error> {
        let args = format!(
            "
                libra-genesis-tool
                operator-key
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={validator_ns}
                --shared-backend backend={backend};\
                    path={path};\
                    namespace={shared_ns}
            ",
            backend = DISK,
            path = self.path_string(),
            validator_ns = validator_ns,
            shared_ns = shared_ns,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.operator_key()
    }

    pub fn owner_key(
        &self,
        validator_ns: &str,
        shared_ns: &str,
    ) -> Result<Ed25519PublicKey, Error> {
        let args = format!(
            "
                libra-genesis-tool
                owner-key
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={validator_ns}
                --shared-backend backend={backend};\
                    path={path};\
                    namespace={shared_ns}
            ",
            backend = DISK,
            path = self.path_string(),
            validator_ns = validator_ns,
            shared_ns = shared_ns,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.owner_key()
    }

    #[cfg(test)]
    pub fn set_layout(&self, path: &str) -> Result<crate::layout::Layout, Error> {
        let args = format!(
            "
                libra-genesis-tool
                set-layout
                --path {path}
                --shared-backend backend={backend};\
                    path={storage_path}
            ",
            path = path,
            backend = DISK,
            storage_path = self.path_string(),
        );

        let command = Command::from_iter(args.split_whitespace());
        command.set_layout()
    }

    pub fn set_operator(&self, operator_name: &str, shared_ns: &str) -> Result<String, Error> {
        let args = format!(
            "
                libra-genesis-tool
                set-operator
                --operator-name {operator_name}
                --shared-backend backend={backend};\
                    path={path};\
                    namespace={shared_ns}
            ",
            operator_name = operator_name,
            backend = DISK,
            path = self.path_string(),
            shared_ns = shared_ns,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.set_operator()
    }

    pub fn treasury_compliance_key(
        &self,
        validator_ns: &str,
        shared_ns: &str,
    ) -> Result<Ed25519PublicKey, Error> {
        let args = format!(
            "
                libra-genesis-tool
                treasury-compliance-key
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={validator_ns}
                --shared-backend backend={backend};\
                    path={path};\
                    namespace={shared_ns}
            ",
            backend = DISK,
            path = self.path_string(),
            validator_ns = validator_ns,
            shared_ns = shared_ns,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.treasury_compliance_key()
    }

    pub fn validator_config(
        &self,
        owner_name: &str,
        validator_address: NetworkAddress,
        fullnode_address: NetworkAddress,
        chain_id: ChainId,
        validator_ns: &str,
        shared_ns: &str,
    ) -> Result<Transaction, Error> {
        let args = format!(
            "
                libra-genesis-tool
                validator-config
                --owner-name {owner_name}
                --validator-address {validator_address}
                --fullnode-address {fullnode_address}
                --chain-id {chain_id}
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={validator_ns}
                --shared-backend backend={backend};\
                    path={path};\
                    namespace={shared_ns}
            ",
            owner_name = owner_name,
            validator_address = validator_address,
            fullnode_address = fullnode_address,
            chain_id = chain_id.id(),
            backend = DISK,
            path = self.path_string(),
            validator_ns = validator_ns,
            shared_ns = shared_ns,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.validator_config()
    }

    #[cfg(test)]
    pub fn verify(&self, namespace: &str) -> Result<String, Error> {
        let args = format!(
            "
                libra-genesis-tool
                verify
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={ns}
            ",
            backend = DISK,
            path = self.path_string(),
            ns = namespace,
        );

        let command = Command::from_iter(args.split_whitespace());
        command.verify()
    }

    pub fn verify_genesis(&self, namespace: &str, genesis_path: &Path) -> Result<String, Error> {
        let args = format!(
            "
                libra-genesis-tool
                verify
                --validator-backend backend={backend};\
                    path={path};\
                    namespace={ns}
                --genesis-path {genesis_path}
            ",
            backend = DISK,
            path = self.path_string(),
            ns = namespace,
            genesis_path = genesis_path.to_str().expect("Unable to parse genesis_path"),
        );

        let command = Command::from_iter(args.split_whitespace());
        command.verify()
    }
}
