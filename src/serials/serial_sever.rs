use async_std::task;
use async_std::{channel::unbounded, channel::Receiver, channel::Sender, task::block_on};
use serialport::SerialPort;
use std::{cell::RefCell, rc::Rc, thread};
use std::time::Duration;

struct SerialSever {
    port: Box<dyn SerialPort>,
}

pub struct SerialController {
	sender: Sender<String>,
	receiver: Receiver<String>,
}

impl SerialController {
	pub fn new(path: String, baud_rate: u32) -> SerialController {
		let (message_sender, ready_to_write) = unbounded::<String>();
        let (send_to_read, message_reader) = unbounded::<String>();
        thread::spawn(move || {
            let serial_sever = Rc::new(RefCell::new(SerialSever::open_serial(
                path.as_str(),
                baud_rate,
            )));
            let serial_sever_clone = serial_sever.clone();
            task::block_on(async {
                let a = read_message(serial_sever, send_to_read);
                let b = write_message(serial_sever_clone, ready_to_write);
                futures::join!(a, b);
            })
        });
		SerialController { sender: message_sender, receiver: message_reader }
	}

	pub fn send(&self, message: String) {
		block_on(self.sender.send(message)).unwrap();
	}
	pub fn read(&self) -> String {
		block_on(self.receiver.recv()).unwrap()
	}
}
impl SerialSever {
    fn open_serial(path: &str, baud_rate: u32) -> Self {
        let port: Box<dyn SerialPort> = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        SerialSever { port: port }
    }
}
async fn read_message(serial_sever: Rc<RefCell<SerialSever>>, sender: Sender<String>) {
    let mut is_error;
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match serial_sever
            .borrow_mut()
            .port
            .read(serial_buf.as_mut_slice())
        {
            Ok(t) => {
                is_error = false;
                let mut str: String = String::new();
                //let mut temp = &mut str;
                for i in serial_buf[0..t].to_vec() {
                    str.push_str(format!("{:02x}", i).to_uppercase().as_str());
                }
                sender.send(str.clone()).await.unwrap();
                //let str: String = format!("{:#X}", str);
                println!("read:{:?}", str);
            }
            Err(_) => {
                is_error = true;
            }
        }
        if is_error {
            task::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}

async fn write_message(serial_sever: Rc<RefCell<SerialSever>>, receiver: Receiver<String>) {
    let mut is_error;
    loop {
        match receiver.recv().await {
            Ok(t) => {
                is_error = false;
                let send = u128::from_str_radix(t.as_str(), 16).unwrap();
                let output = &send.to_be_bytes()[send.to_be_bytes().len() - t.len() / 2..];
                serial_sever.borrow_mut().port.write(output).expect("Write failed!");
            }
            Err(_) => {
                is_error = true;
            }
        }
        if is_error {
            task::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}
