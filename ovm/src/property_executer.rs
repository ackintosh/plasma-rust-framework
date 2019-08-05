use crate::deciders::{AndDecider, ForAllSuchThatDecider, NotDecider, PreimageExistsDecider};
use crate::error::Error;
use crate::quantifiers::IntegerRangeQuantifier;
use crate::types::Decider;
use crate::types::{Decision, Property, Quantifier, QuantifierResult};
use bytes::Bytes;
use plasma_db::impls::kvs::CoreDbLevelDbImpl;
use plasma_db::traits::db::DatabaseTrait;

pub struct PropertyExecuter {
    db: CoreDbLevelDbImpl,
}

impl Default for PropertyExecuter {
    fn default() -> Self {
        PropertyExecuter {
            db: CoreDbLevelDbImpl::open("test"),
        }
    }
}

impl PropertyExecuter {
    pub fn get_db(&self) -> &CoreDbLevelDbImpl {
        &self.db
    }
    pub fn decide(&self, property: &Property, witness: Option<&Bytes>) -> Result<Decision, Error> {
        match property {
            Property::AndDecider(input) => AndDecider::decide(self, input, witness),
            Property::NotDecider(input) => NotDecider::decide(self, input, witness),
            Property::PreimageExistsDecider(input) => {
                PreimageExistsDecider::decide(self, input, witness)
            }
            Property::ForAllSuchThatDecider(input) => {
                ForAllSuchThatDecider::decide(self, input, witness)
            }
            _ => panic!("not implemented!!"),
        }
    }
    pub fn get_all_quantified(&self, quantifier: &Quantifier) -> QuantifierResult {
        match quantifier {
            Quantifier::IntegerRangeQuantifier(start, end) => {
                IntegerRangeQuantifier::get_all_quantified(*start, *end)
            }
            _ => panic!("not implemented!!"),
        }
    }
}
