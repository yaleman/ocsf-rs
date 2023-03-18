#[test]
pub fn print_the_version() {
    use ocsf::OCSF_SCHEMA_VERSION;
    println!("{}", OCSF_SCHEMA_VERSION);
}
