mod configs;
mod serials;

use serials::serial_sever::SerialSever;
use configs::read_ini::read_config;
use slint::SharedString;

//引入模块
slint::include_modules!();

fn main() {
	println!("{:?}" ,read_config()) ;
    let mut serial_sever = SerialSever::open_serial("COM3",9600);
	
	serial_sever.send_message("EF0101FF", 4);
	//serial_sever.read_message();
	let App =MainWindow::new().unwrap();
	let weak1 = App.as_weak();// as_weak避免内存泄露
	App.get_send_message();
	App.set_send_message(SharedString::from("EF0101FF"));
	// 重写display_message方法
	App.on_display_message(move || {
		let recipe = weak1.upgrade().unwrap();
		recipe.set_send_message(SharedString::from(String::from("clicked: ")+&recipe.get_time().to_string()));
	});
	App.run().unwrap();
}
