use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;

struct LEDRecord {
    index: usize,
    color: (u8, u8, u8),
}

fn write_frame(port: &mut Box<dyn SerialPort>, records: &[LEDRecord]) -> std::io::Result<()> {
    let mut frame = String::from("START;");
    for record in records {
        frame.push_str(&format!("{},{},{},{};", record.index, record.color.0, record.color.1, record.color.2));
    }
    frame.push_str("END\n");

    port.write_all(frame.as_bytes())?;
    port.flush()?;

    Ok(())
}

fn main() {
    let port_name = "COM11";

    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");
    
    std::thread::sleep(Duration::from_millis(2000));
    
    let mut buffer: Vec<LEDRecord> = Vec::new();
    
    let mut counter: i32 = 0;
    let mut direction: i32 = 1;

    loop {
        counter += direction;
        if counter >= 255 || counter <= 0 {
            direction *= -1;
        }
        buffer.clear();

        for i in 0..27 {
            buffer.push(LEDRecord {
                index: i,
                color: (0u8, counter as u8, 0u8),
            });
        }

        write_frame(&mut port, &buffer).unwrap();
    }
}