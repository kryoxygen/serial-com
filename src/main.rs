mod configs;
mod serials;

use configs::read_ini::read_config;
use serials::serial_sever::SerialSever;
use slint::SharedString;
use std::{thread, sync::{Arc, Mutex}};

//引入模块
slint::include_modules!();

fn main() {
    println!("{:?}", read_config());
	let shared_variable = Arc::new(Mutex::new(Vec::new()));
    let mut serial_sever: SerialSever = SerialSever::open_serial("COM3", 9600);
    let App = MainWindow::new().unwrap();
    let weak1 = App.as_weak(); // as_weak避免内存泄露
    let weak2 = App.as_weak(); // as_weak避免内存泄露
    App.get_send_message();
    App.set_send_message(SharedString::from("123456"));
    //重写display_message方法
    App.on_display_message(move || {
    	let recipe = weak1.upgrade().unwrap();
    	println!("display_message: {}", recipe.get_send_message());
    	//recipe.set_send_message(SharedString::from(String::from_utf8_lossy(&data).as_ref()));
    });
	let shared_variable_clone = shared_variable.clone();
	// 将指定函数添加到内部队列，通知事件循环唤醒。一旦被唤醒，任何排队的函子都将被调用。
	thread::spawn(move || {
        serial_sever.send_message("EF0101FF", 4);
        loop {
            let mut temp: Vec<u8> = vec![0; 32];
			let handle_copy = weak2.clone();
            match serial_sever.read_message() {
                Ok(t) => {
                   
                    temp = t;
					let mut shared_variable = shared_variable_clone.lock().unwrap();
					*shared_variable= temp.clone();
					println!("shared:{:?}", shared_variable);
					slint::invoke_from_event_loop(move || handle_copy.unwrap().set_send_message(SharedString::from(String::from_utf8_lossy(&temp).as_ref())));
                }
                Err(_) => {
                    ();
                }
            }
        }
    });
    App.run().unwrap();
}
