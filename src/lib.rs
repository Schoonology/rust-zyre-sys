#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{ CStr, CString };
    use super::*;

    #[test]
    fn uuid_length_test() {
        unsafe {
            let mut engine = zyre_new(CString::new("undecided").unwrap().as_ptr());
            let uuid = CStr::from_ptr(zyre_uuid(engine)).to_str().unwrap();
            zyre_destroy(&mut engine);

            assert_eq!(uuid.len(), 32);
        }
    }
}
