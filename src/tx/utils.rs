// util

use tx::tx::{Transaction};
use tx::output::{TXOutput};
use tx::input::{TXInput};
use tx::utxo_set::UTXOSet;
use wallet::utils::load_account;

pub fn new_coinbase_tx(to: Vec<u8>, _data: String) -> Transaction {
    let subsidy = 10;
    let txin = TXInput {
        txid: "".to_string().into_bytes(),
        vout_idx: -1,
        signature: vec![],
        pub_key: to.to_owned()
    };
    let txout = TXOutput{
        value: subsidy,
        pubkey_hash: to,
    };
    let tx = Transaction {
        id: vec![],
        vin: vec![txin],
        vout: vec![txout]
    };
    
    return tx;
}

pub fn new_utxo_transaction(_to: Vec<u8>, _from: Vec<u8>, _amount: i32, _utxo_set: UTXOSet) -> Option<Transaction> {
    println!("new utxo transaction...");
    let mut _inputs: Vec<TXInput> = vec![];
    let mut _outputs: Vec<TXOutput> = vec![];
    let _bc = _utxo_set.to_owned().blockchain;

    let (_acc, _valid_outputs) = _utxo_set.find_spendable_outputs(_from.to_owned(), _amount);
    println!("\npack txinputs...");
    
    for (_txid, _outs) in _valid_outputs.clone().iter() {
        let _tx_id = _txid.to_owned().into_bytes();

        for out in _outs {
            let _input = TXInput{
                txid: _tx_id.to_owned(),
                vout_idx: out.to_owned(),
                signature: vec![],
                pub_key: load_account().pub_key
            };
            _inputs.append(&mut vec![_input]);
        }
    }
    
    if _acc < _amount { return None; }

    _outputs.append(&mut vec![TXOutput{
        value: _amount,
        pubkey_hash: _to
    }]);
    
    
    _outputs.append(&mut vec![TXOutput{
        value:  -_amount,
        pubkey_hash: _from,
    }]);
    

    let mut _tx = Transaction{ id: vec![], vin: _inputs, vout: _outputs };
    _tx = _tx.set_id();
    _bc.sign_transaction(_tx.to_owned(), load_account().priv_key);
    
    return Some(_tx);
}
