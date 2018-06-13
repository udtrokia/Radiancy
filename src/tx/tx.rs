// Tx
use bincode::serialize;
    
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Vec<u8>,
    vin: Vec<u8>, //TXInput,
    vout: Vec<u8>, //TXOutput,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct TXOutput {
    value: i32,
    script_pubkey: String
}
#[derive(Clone, Serialize, Deserialize, Debug)]
struct TXInput {
    txid: Vec<u8>,
    vout: i32,
    script_sig: String
}

impl TXInput {
    pub fn can_unlock_output_with(self, unlocking_data: String) -> bool {
        return self.script_sig == unlocking_data;
    }
}

impl TXOutput {
    pub fn can_be_unlocked_with(self, unlocking_data: String) -> bool {
        return self.script_pubkey == unlocking_data;
    }
}


pub fn new_coinbase_tx(to: String, mut data: String) -> Transaction {
    if data == "".to_string() {
        data = "Reward to ".to_string() + &to;
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

