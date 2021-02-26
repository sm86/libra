//! account: alice
//! account: bob

//! new-transaction
//! sender: diemroot
script {
use 0x1::DiemAccount;
use 0x1::Coin1::Coin1;
fun main(account: &signer) {
    DiemAccount::make_payment<Coin1>(
        {{alice}},
        {{bob}},
        100,
        x"deadbeef",
        x"",
        account
    );
}
}

// check: SentPaymentEvent
// check: deadbeef
// check: ReceivedPaymentEvent
// check: deadbeef
// check: "Keep(EXECUTED)"
