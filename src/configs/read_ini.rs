use ini::Ini;
#[warn(unused_parens)]
pub struct Config {
	pub port : String,
	pub baud : u32,
}
pub fn read_config() -> Config {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let section = conf.section(Some("serial")).unwrap();
    let port = section.get("port").unwrap().to_string();
    let baud = section.get("baud").unwrap().parse::<u32>().unwrap();
    Config { port, baud }
}