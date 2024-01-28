use async_std::task;
use async_std::{channel::unbounded, channel::Receiver, channel::Sender, task::block_on};
use serialport::{Error, SerialPort};
use std::{cell::RefCell, rc::Rc, thread};
use std::{io, time::Duration};

pub struct SerialSever {
    port: Box<dyn SerialPort>,
}
impl SerialSever {
    pub fn new(path: String, baud_rate: u32)-> (Sender<String>, Receiver<String>) {
        let (message_sender, ready_queue) = unbounded::<String>();
        let (message_read, read_queue) = unbounded::<String>();
        thread::spawn(move || {
            let serial_sever = Rc::new(RefCell::new(SerialSever::open_serial(path.as_str(), baud_rate)));
            let serial_sever_clone = serial_sever.clone();
            task::block_on(async {
                let a = read_message(serial_sever, message_read);
                let b = write_message(serial_sever_clone, ready_queue);
                futures::join!(a, b);
            })
        });
		(message_sender, read_queue)
    }

    fn open_serial(path: &str, baud_rate: u32) -> Self {
        let port: Box<dyn SerialPort> = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        SerialSever { port: port }
    }
    // 发送串口数据
    fn send_message(&mut self, message: &str, size: usize) {
        let send = u128::from_str_radix(message, 16).unwrap();
        let output = &send.to_be_bytes()[send.to_be_bytes().len() - size..];
        self.port.write(output).expect("Write failed!");
    }
    // 读取串口数据
    pub fn read_message(&mut self) -> Result<Vec<u8>, ()> {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match self.port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                println!("size:{:?}", t);
                Ok(serial_buf[0..t].to_vec())
            }
            //Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => Err(()),
            Err(e) => Err(eprintln!("{:?}", e)),
        }
    }
}
pub async fn read_message(serial_sever: Rc<RefCell<SerialSever>>, sender: Sender<String>) {
    let mut is_error;
    loop {
        match serial_sever.borrow_mut().read_message() {
            Ok(t) => {
                is_error = false;
                let mut str: String = String::new();
                //let mut temp = &mut str;
                for i in t {
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

pub async fn write_message(serial_sever: Rc<RefCell<SerialSever>>, receiver: Receiver<String>) {
    let mut is_error;
    loop {
        match receiver.recv().await {
            Ok(t) => {
                is_error = false;
                serial_sever.borrow_mut().send_message(t.as_str(), t.len() / 2);
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
