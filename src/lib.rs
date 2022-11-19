#![feature(once_cell)]
#![feature(const_cstr_methods)]
pub mod ffi;
pub mod runtime;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
