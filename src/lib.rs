#![feature(once_cell)]
#![feature(const_cstr_methods)]
#![feature(slice_ptr_get)]
#![feature(try_blocks)]
pub mod ffi;
pub mod runtime;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn size_of_option_classkey() {
        assert_eq!(
            std::mem::size_of::<super::runtime::context::ClassKey>(),
            std::mem::size_of::<
                Result<
                    super::runtime::context::ClassKey,
                    (super::runtime::context::ClassKey, bool),
                >,
            >(),
        )
    }
}
