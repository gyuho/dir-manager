use dirs::home_dir;

pub fn named(name: &str, ext: Option<&str>) -> String {
    let home = home_dir().unwrap();
    let file_path = home.join(format!("{}{}", name, ext.unwrap_or("")));
    let file_path = file_path.as_os_str().to_str().unwrap();
    String::from(file_path)
}

/// RUST_LOG=debug cargo test --all-features --package dir-manager --lib -- home::test_home_dir --exact --show-output
#[test]
fn test_home_dir() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    log::info!("{}", named("hello", Some(".txt")));
}
