extern crate ethereum_types;
extern crate rlp;

use super::state_object::StateObject;
use ethereum_types::Address;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

pub struct StateUpdate {
    pub start: u64,
    pub end: u64,
    pub block: u64,
    pub plasma_contract: Address,
    pub new_state: StateObject,
}

impl Encodable for StateUpdate {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(5);
        s.append(&self.start);
        s.append(&self.end);
        s.append(&self.block);
        s.append(&self.plasma_contract);
        s.append(&self.new_state);
    }
}

impl Decodable for StateUpdate {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let new_state_result: Result<StateObject, DecoderError> = rlp.val_at(4);
        return new_state_result.map(|new_state| {
            return StateUpdate {
                start: rlp.val_at(0).unwrap_or(0),
                end: rlp.val_at(1).unwrap_or(0),
                block: rlp.val_at(2).unwrap_or(0),
                plasma_contract: rlp.val_at(3).unwrap_or(Address::zero()),
                new_state: new_state,
            };
        });
    }
}

#[cfg(test)]
mod tests {
    use super::DecoderError;
    use super::StateObject;
    use super::StateUpdate;
    use bytes::Bytes;
    use ethereum_types::Address;

    #[test]
    fn test_rlp_encode() {
        let message = "parameters".as_bytes();
        let message_bytes = Bytes::from(message);
        let state_object = StateObject {
            predicate: Address::zero(),
            parameters: message_bytes,
        };
        let state_update = StateUpdate {
            start: 0,
            end: 0,
            block: 0,
            plasma_contract: Address::zero(),
            new_state: state_object,
        };
        let encoded = rlp::encode(&state_update);
        let _decoded: StateUpdate = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.start, state_update.start);
    }

    #[test]
    fn fail_to_decode() {
        let failtodecode = "failtodecode";
        let encoded = rlp::encode(&failtodecode);
        let decoded: Result<StateUpdate, DecoderError> = rlp::decode(&encoded);
        assert_eq!(decoded.is_err(), true);
    }
}
