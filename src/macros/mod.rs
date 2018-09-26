#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! write_string {
        ($w:expr) => {
            $w.to_string()
        };
    }
}
