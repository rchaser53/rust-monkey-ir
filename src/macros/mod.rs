#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! write_string {
        ($w:expr) => {
            $w.to_string()
        };
    }

    #[macro_export]
    macro_rules! c_string {
        ($w:expr) => {
            CString::new($w).unwrap()
        };
    }

    #[macro_export]
    macro_rules! llvm_integer {
        ($value:expr) => {
          const_int(int32_type(), $value)
        };
    }
}
