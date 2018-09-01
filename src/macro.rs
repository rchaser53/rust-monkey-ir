#[macro_export]
macro_rules! write_string {
  ($w:expr) => ( $w.to_string() );
}