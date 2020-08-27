// Adding new validator epoch info

//! new-transaction
//! sender: association
script{
use 0x0::ValidatorUniverse;
use 0x0::Vector;
use 0x0::Transaction;

fun main(account: &signer) {
    // NOTE: in functional and e2e tests the genesis block includes 3 validators.
    // this is set here anguage/tools/vm-genesis/src/lib.rs
    ValidatorUniverse::add_validator(0xDEADBEEF);
    let validators_in_genesis = 4;
    let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(account));

    Transaction::assert(len == (validators_in_genesis + 1), 100001);
}
}
// check: EXECUTED