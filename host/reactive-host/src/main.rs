use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;

struct LEDRecord {
    index: usize,
    color: (u8, u8, u8),
}

fn main() {
    let port_name = "COM11";

    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");
    
    std::thread::sleep(Duration::from_millis(2000));
    
    let mut buffer: Vec<LEDRecord> = Vec::new();
    
    for i in 0..27 {
        buffer.push(LEDRecord {
            index: i,
            color: (10, 255, 10),
        });
    }

    let mut frame = String::from("START;");
    for record in buffer {
        frame.push_str(&format!("{},{},{},{};", record.index, record.color.0, record.color.1, record.color.2));
    }
    frame.push_str("END\n");

    port.write_all(frame.as_bytes()).unwrap();
    port.flush().unwrap();
}