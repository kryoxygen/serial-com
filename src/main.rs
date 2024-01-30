mod configs;
mod serials;

use configs::read_ini::read_config;
use serials::serial_sever::{SerialController};
use slint::SharedString;
//引入模块
slint::include_modules!();

fn main() {
    let config = read_config();
	//let shared_variable = Arc::new(Mutex::new(Vec::new()));
    //let mut serial_sever = SerialSever::open_serial("COM3", 9600);
	// let (message_sender, ready_queue) = unbounded::<String>();
	// let (message_read, read_queue) = unbounded::<String>();
	// thread::spawn(move || {
	// 	let serial_sever = Rc::new(RefCell::new(SerialSever::open_serial(config.port.as_str(), config.baud)));
	// 	let serial_sever_clone = serial_sever.clone();
	// 	task::block_on(async {
	// 	let a = read_message(serial_sever,message_read);
	// 	let b = write_message(serial_sever_clone, ready_queue);
	// 	futures::join!(a, b);
	// 	})
	// });
	let serial_controller = SerialController::new(config.port, config.baud);
    //serial_sever.0.send("AA01FFEE".to_string());
	serial_controller.send("AA01FFEE".to_string());
	let App = MainWindow::new().unwrap();
    let weak1 = App.as_weak(); // as_weak避免内存泄露
    let weak2 = App.as_weak(); // as_weak避免内存泄露
    App.get_send_message();
    App.set_send_message(SharedString::from("123456"));
	// block_on(message_sender.send("AA03FFEE")).unwrap();
    //重写display_message方法
    App.on_display_message(move || {
		serial_controller.send("AA01FFEE".to_string());
		let data = serial_controller.read();
		// block_on(message_sender.send("AA03FFEE")).unwrap();
    	let recipe = weak1.upgrade().unwrap();
    	//println!("display_message: {}", recipe.get_send_message());
    	//println!("display_message: {}",block_on(read_queue.recv()).unwrap());
    	recipe.set_send_message(SharedString::from(data));
    });
	//let shared_variable_clone = shared_variable.clone();
	// 将指定函数添加到内部队列，通知事件循环唤醒。一旦被唤醒，任何排队的函子都将被调用。
	// thread::spawn(move || {
    //     serial_sever.send_message("EF0101FF", 4);
    //     loop {
    //         let mut temp: Vec<u8> = vec![0; 32];
	// 		let handle_copy = weak2.clone();
    //         match serial_sever.read_message() {
    //             Ok(t) => {
                   
    //                 temp = t;
	// 				let mut shared_variable = shared_variable_clone.lock().unwrap();
	// 				*shared_variable= temp.clone();
	// 				println!("shared:{:?}", shared_variable);
	// 				slint::invoke_from_event_loop(move || handle_copy.unwrap().set_send_message(SharedString::from(String::from_utf8_lossy(&temp).as_ref())));
    //             }
    //             Err(_) => {
    //                 ();
    //             }
    //         }
    //     }
    // });
    App.run().unwrap();
}

