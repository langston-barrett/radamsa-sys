#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe { radamsa_init() };
        const SIZE: usize = 1024;
        let mut out_vec = vec![0u8; SIZE];
        let mut input = "hello world!".bytes().collect::<Vec<_>>();
        for _ in 0..4 {
            let out_len = unsafe {
                radamsa(
                    input.as_mut_ptr(),
                    input.len(),
                    out_vec.as_mut_ptr(),
                    SIZE,
                    0,
                )
            };
            assert!(out_len <= SIZE);
        }
    }
}
