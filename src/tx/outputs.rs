use tx::output::TXOutput;
use bincode::{serialize, deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>
}

impl TXOutputs {
    pub fn s(self) -> Vec<u8> {
        let _result = serialize(&self);
        assert_eq!(_result.is_ok(), true);
        return _result.unwrap();
    }
    
    pub fn ds(s_outputs: Vec<u8>) -> TXOutputs {
        let _result = deserialize(&s_outputs);
        if _result.is_err() {return TXOutputs{outputs: vec![]}};
        assert_eq!(_result.is_ok(), true);
        return _result.unwrap();
    }
}

