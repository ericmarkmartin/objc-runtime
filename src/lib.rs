#![feature(const_cstr_methods)]
#![feature(slice_ptr_get)]
#![feature(try_blocks)]
#![feature(c_variadic)]
pub mod ffi;
mod runtime;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
