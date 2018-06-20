use tx::input::TXInput;
use bincode::{serialize, deserialize};
    
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXInputs {
    pub inputs: Vec<TXInput>,
}

impl TXInputs {
    pub fn s(self) -> Vec<u8> {
        let _result = serialize(&self);
        assert_eq!(_result.is_ok(), true);
        return _result.unwrap();
    }
    pub fn ds(s_inputs: Vec<u8>) -> TXInputs {
        let _result = deserialize(&s_inputs);
        assert_eq!(_result.is_ok(), true);
        return _result.unwrap();
    }
}
