// Tx
use bincode::serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Transaction {
    id: Vec<u8>,
    vin: Vec<u8>, //TXInput,
    vout: Vec<u8>, //TXOutput,
}

struct TXOutput {
    value: i32,
    script_pubkey: String
}

struct TXInput {
    txid: Vec<u8>,
    vout: i32,
    script_sig: String
}

fn new_coinbase_tx(to: String, mut data: String) -> Transaction {
    if data == "".to_string() {
        data = "Reward to ".to_string() + &to
    }
    let subsidy = 1;
    let txin = TXInput {
        txid: vec![],
        vout: -1,
        script_sig: data,
    };
    let txout = TXOutput{
        value: subsidy,
        script_pubkey: to,
    };
    let tx = Transaction {
        id: vec![],
        vin: serialize(&txin).unwrap(),
        vout: serialize(&txout).unwrap(),
    };

    return tx;
}
