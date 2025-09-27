#[macro_export]
macro_rules! load_test_asset {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets", $path))
    };
}
