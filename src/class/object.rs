use std::convert::From;

use types::Value;

use super::array::Array;
use super::class::Class;
use super::fixnum::Fixnum;
use super::hash::Hash;
use super::string::RString;

use super::traits::RawObject;

#[derive(Clone)]
pub struct Object {
    value: Value
}

impl Object {
    pub fn as_array(&self) -> Array {
        Array::from(self.value())
    }

    pub fn as_class(&self) -> Class {
        Class::from(self.value())
    }

    pub fn as_fixnum(&self) -> Fixnum {
        Fixnum::from(self.value())
    }

    pub fn as_hash(&self) -> Hash {
        Hash::from(self.value())
    }

    pub fn as_string(&self) -> RString {
        RString::from(self.value())
    }
}

impl From<Value> for Object {
    fn from(value: Value) -> Self {
        Object {
            value: value
        }
    }
}

impl RawObject for Object {
    fn value(&self) -> Value {
        self.value
    }
}
