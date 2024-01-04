use serialport::SerialPort;
use std::{io, time::Duration};
use std::thread;
pub struct SerialSever {
    port: Box<dyn SerialPort>,
}
impl SerialSever {
    pub fn open_serial(path: &str, baud_rate: u32) -> Self {
        let port: Box<dyn SerialPort> = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        SerialSever { port: port }
    }
    // 发送串口数据
    pub fn send_message(&mut self, message: &str, size: usize) {
        let send = u128::from_str_radix(message, 16).unwrap();
        let output = &send.to_be_bytes()[send.to_be_bytes().len() - size..];

        self.port.write(output).expect("Write failed!");
    }
    // 读取串口数据
    pub fn read_message(&mut self) {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        loop {
            match self.port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                //Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
}
