use super::abi::{Decodable, Encodable};
use super::error::{Error, ErrorKind};
use super::StateUpdate;
use bytes::Bytes;
use ethabi::Token;
use ethereum_types::Address;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// StateQuery is the query to get StateUpdates
#[derive(Debug, Clone, Hash)]
pub struct StateQuery {
    plasma_contract: Address,
    predicate_address: Address,
    start: Option<u64>,
    end: Option<u64>,
    params: Bytes,
}

impl StateQuery {
    pub fn new(
        plasma_contract: Address,
        predicate_address: Address,
        start: Option<u64>,
        end: Option<u64>,
        params: Bytes,
    ) -> Self {
        Self {
            plasma_contract,
            predicate_address,
            start,
            end,
            params,
        }
    }
    pub fn get_plasma_contract(&self) -> Address {
        self.plasma_contract
    }
    pub fn get_predicate_address(&self) -> Address {
        self.predicate_address
    }
    pub fn get_start(&self) -> Option<u64> {
        self.start
    }
    pub fn get_end(&self) -> Option<u64> {
        self.end
    }
    pub fn get_params(&self) -> &[u8] {
        &self.params
    }
    pub fn to_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Encodable for StateQuery {
    fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }
    fn to_tuple(&self) -> Vec<Token> {
        vec![
            Token::Address(self.plasma_contract),
            Token::Address(self.predicate_address),
            Token::Int(self.start.unwrap().into()),
            Token::Int(self.end.unwrap().into()),
            Token::Bytes(self.params.to_vec()),
        ]
    }
}

impl Decodable for StateQuery {
    type Ok = Self;
    fn from_tuple(tuple: &[Token]) -> Result<Self, Error> {
        let plasma = tuple[0].clone().to_address();
        let predicate = tuple[1].clone().to_address();
        let start = tuple[2].clone().to_uint();
        let end = tuple[3].clone().to_uint();
        let params = tuple[4].clone().to_bytes();
        if let (Some(plasma), Some(predicate), Some(params)) = (plasma, predicate, params) {
            Ok(StateQuery::new(
                plasma,
                predicate,
                start.map(|s| s.as_u64()),
                end.map(|e| e.as_u64()),
                Bytes::from(params),
            ))
        } else {
            Err(Error::from(ErrorKind::AbiDecode))
        }
    }
    fn from_abi(data: &[u8]) -> Result<Self, Error> {
        let decoded: Vec<Token> = ethabi::decode(
            &[
                ethabi::ParamType::Address,
                ethabi::ParamType::Address,
                ethabi::ParamType::Int(8),
                ethabi::ParamType::Int(8),
                ethabi::ParamType::Bytes,
            ],
            data,
        )
        .map_err(|_e| Error::from(ErrorKind::AbiDecode))?;
        Self::from_tuple(&decoded)
    }
}

/// The result of StateQuery
#[derive(Debug, Clone)]
pub struct StateQueryResult {
    state_update: StateUpdate,
    result: Vec<Bytes>,
}

impl StateQueryResult {
    pub fn new(state_update: StateUpdate, result: &[Bytes]) -> Self {
        Self {
            state_update,
            result: result.to_vec(),
        }
    }
    pub fn get_state_update(&self) -> &StateUpdate {
        &self.state_update
    }
    pub fn get_result(&self) -> &[Bytes] {
        &self.result
    }
    pub fn to_tuple(&self) -> Vec<Token> {
        vec![
            Token::Bytes(self.state_update.to_abi()),
            Token::Array(
                self.result
                    .iter()
                    .map(|r| Token::Bytes(r.to_vec()))
                    .collect(),
            ),
        ]
    }
    pub fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&self.to_tuple())
    }
    pub fn from_tuple(tuple: &[Token]) -> Result<Self, Error> {
        let state_update_bytes = tuple[0].clone().to_bytes();
        let result_array = tuple[1].clone().to_array();
        if let (Some(state_update_bytes), Some(result_array)) = (state_update_bytes, result_array) {
            let result: Vec<Bytes> = result_array
                .iter()
                .filter_map(|r| r.clone().to_bytes().map(Bytes::from))
                .collect();
            Ok(StateQueryResult::new(
                StateUpdate::from_abi(&state_update_bytes).ok().unwrap(),
                &result,
            ))
        } else {
            Err(Error::from(ErrorKind::AbiDecode))
        }
    }
    pub fn from_abi(data: &[u8]) -> Result<Self, Error> {
        let decoded: Vec<Token> = ethabi::decode(
            &[
                ethabi::ParamType::Bytes,
                ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
            ],
            data,
        )
        .map_err(|_e| Error::from(ErrorKind::AbiDecode))?;
        Self::from_tuple(&decoded)
    }
}
