use types::Value;
use unsafe_binding::string::{rb_str_new_cstr, rb_string_value_cstr};
use util::{cstr_as_string, str_as_ptr};

pub fn new(string: &str) -> Value {
    unsafe {
        rb_str_new_cstr(str_as_ptr(string))
    }
}

pub fn from_value(value: Value) -> String {
    unsafe {
        let str = rb_string_value_cstr(&value);

        cstr_as_string(str)
    }
}
