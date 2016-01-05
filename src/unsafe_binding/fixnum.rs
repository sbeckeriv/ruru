use types::{c_long, SignedValue, Value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_int2inum(num: SignedValue) -> Value;
    pub fn rb_num2int(num: Value) -> c_long;
}
