// util

use tx::tx::{Transaction};
use tx::output::{TXOutput};
use tx::input::{TXInput};
use tx::utxo_set::UTXOSet;

pub fn new_coinbase_tx(to: Vec<u8>, _data: String) -> Transaction {
    let subsidy = 1;
    let txin = TXInput {
        txid: "coinbase_transaction_id".to_string().into_bytes(),
        vout_idx: -1,
        signature: vec![],
        pub_key: to.to_owned()
    };
    let txout = TXOutput{
        value: subsidy,
        pubkey_hash: to,
    };
    let tx = Transaction {
        id: "coinbase".to_string().into_bytes(),
        vin: vec![txin],
        vout: vec![txout]
    };

    return tx;
}

pub fn new_utxo_transaction(_to: Vec<u8>, _from: Vec<u8>, _amount: i32, _utxo_set: UTXOSet) -> Option<Transaction> {
    let mut _inputs: Vec<TXInput> = vec![];
    let mut _outputs: Vec<TXOutput> = vec![];
    let _bc = _utxo_set.blockchain;
    let (_acc, _valid_outputs) = _bc.find_spendable_outputs(_from.to_owned(), _amount);

    for (_txid, _outs) in _valid_outputs.clone().iter() {
        let _tx_id = _txid.to_owned().into_bytes();
        
        for out in _outs {
            let _input = TXInput{
                txid: _tx_id.to_owned(),
                vout_idx: out.to_owned(),
                signature: _from.to_owned(),
                pub_key: vec![]
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
    //_bc.sign_transaction(_tx.to_owned(), wallet.priv_key);
    
    return Some(_tx);
}
