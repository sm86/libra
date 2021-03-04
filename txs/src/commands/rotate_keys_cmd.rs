//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use std::convert::TryInto;

use libra_types::{
    account_address::AccountAddress, waypoint::Waypoint, 
};
use libra_types::transaction::{
    authenticator::AuthenticationKey
};

use abscissa_core::{Command, Options, Runnable};
use crate::{prelude::app_config, submit_tx::{
        submit_tx,
        get_params_experiment,
        eval_tx_status,
    }};


/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct RotateKeysCmd {
    #[options(help = "force account address, in case of mismatches")]
    force_address: Option<AccountAddress>,
    #[options(help = "url to send txs")]
    url: Option<String>,
    #[options(help = "waypoint to connect to")]
    waypoint: Option<String>,

}

impl Runnable for RotateKeysCmd {    

    fn run(&self) {
        // Use TBD Account, and known AUTHKEY
        // USE EVE keys for signing.
        let url = "http://167.172.248.37:8080";
        let wp = "46357702:6797487d36ec68db37e32d7cd6382d23a7724aaedbf0af85b1fa914eed9c9cd0";
        // TODO(GS): generalize to main.rs for all commands
        // let miner_configs = app_config();
        let mut tx_params = get_params_experiment(wp, url).unwrap();

        tx_params.address = AccountAddress::from_hex_literal("0xf0bfe5b1f34dc012fc9bc369a38f7107").unwrap();
        let new_key = tx_params.auth_key.clone();
        let encoded = hex::encode(tx_params.auth_key.prefix());
        dbg!(&encoded);
        let test_key = tx_params.auth_key.prefix().iter().chain(&tx_params.address.to_u8()).collect();

        dbg!(&test_key);

        let onchain_authkey = AuthenticationKey::new([
            236,
            197,
            154,
            74,
            9,
            99,
            246,
            91,
            94,
            206,
            176,
            255,
            250,
            1,
            234,
            153,
            240,
            191,
            229,
            177,
            243,
            77,
            192,
            18,
            252,
            155,
            195,
            105,
            163,
            143,
            113,
            7,
        ]);

        tx_params.auth_key = AuthenticationKey::new(test_key); //onchain_authkey;
        // let account_json = self.account_json_path.to_str().unwrap();
        match submit_tx(
            &tx_params, 
            transaction_builder::encode_rotate_authentication_key_script(new_key.to_vec())
        ) {
            Err(err) => { println!("{:?}", err) }
            Ok(res)  => {
                eval_tx_status(res);
            }
        }
    }
}