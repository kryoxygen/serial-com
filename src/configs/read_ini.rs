use ini::Ini;
#[warn(unused_parens)]
pub fn read_config() -> String {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let section = conf.section(Some("serial")).unwrap();
    let port = section.get("port").unwrap();
    let baud = section.get("baud").unwrap();
    port.to_string()+":"+baud
}