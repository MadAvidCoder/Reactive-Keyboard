use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;
use ringbuf::HeapRb;
use ringbuf::traits::Split;
use ringbuf::traits::{Producer, Consumer};

#[derive(Clone, Copy)]
struct LEDRecord {
    index: usize,
    color: (u8, u8, u8),
}

const WINDOW: usize = 2048;
const HOP: usize = 512;

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
    
    let mut leds: [LEDRecord; 27] = [LEDRecord { index: 0, color: (0u8, 0u8, 0u8) }; 27];

    let rb = HeapRb::<f32>::new(48000usize * 2usize);
    let (mut producer, mut consumer) = rb.split();

    // let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
    let (sr_tx, mut sr_rx) = tokio::sync::mpsc::channel(1024);

    std::thread::spawn(move || {
        wasapi::initialize_mta().unwrap();
        
        let enumerator = wasapi::DeviceEnumerator::new().unwrap();    
        let device = enumerator.get_default_device(&wasapi::Direction::Render).unwrap();

        let mut audio_client = device.get_iaudioclient().unwrap();
        let format = audio_client.get_mixformat().unwrap();

        sr_tx.blocking_send(format.get_samplespersec() as u32).unwrap();

        audio_client.initialize_client(
            &format,
            &wasapi::Direction::Capture,
            &wasapi::StreamMode::PollingShared {
                autoconvert: true,
                buffer_duration_hns: 100000,
            },
        ).unwrap();

        let capture_client = audio_client.get_audiocaptureclient().unwrap();
        audio_client.start_stream().unwrap();
        
        let mut buf: Vec<u8> = Vec::new();

        loop {
            loop {
                let Some(packet_size) = (match capture_client.get_next_packet_size() {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        continue;
                    }
                }) else { continue; };

                if packet_size == 0 { break; }

                let bytes_per_frame = format.get_nchannels() as usize * (format.get_validbitspersample() as usize / 8);
                let needed = packet_size as usize * bytes_per_frame;

                if buf.len() < needed {
                    buf.resize(needed, 0);
                }

                let Ok((frames_read, _)) = capture_client.read_from_device(&mut buf) else { continue; };
                let bytes_read = frames_read as usize * bytes_per_frame;
                let raw_bytes = &buf[..bytes_read];

                match format.get_subformat().unwrap() {
                    wasapi::SampleType::Float => unsafe {
                        let slice = std::slice::from_raw_parts(
                            raw_bytes.as_ptr() as *const f32,
                            bytes_read / 4,
                        );
                        for frame in slice.chunks(format.get_nchannels() as usize) {
                            let mono = frame.iter().sum::<f32>() / frame.len() as f32;
                            let _ = producer.try_push(mono);
                        }
                    },
                    wasapi::SampleType::Int => unsafe {
                        let slice = std::slice::from_raw_parts(
                            raw_bytes.as_ptr() as *const i16,
                            bytes_read / 2,
                        );
                        for frame in slice.chunks(format.get_nchannels() as usize) {
                            let mono = frame
                                .iter()
                                .map(|&v| v as f32 / i16::MAX as f32)
                                .sum::<f32>()
                                / frame.len() as f32;
                            let _ = producer.try_push(mono);
                        }
                    },
                };
            }
            std::thread::yield_now();
        }
    });

    std::thread::spawn(move || {
        let sample_rate = sr_rx.blocking_recv().unwrap_or(44100);

        let mut buffer = vec![0f32; WINDOW];
        let mut temp = Vec::with_capacity(WINDOW);

        loop {
            while temp.len() < WINDOW {
                match consumer.try_pop() {
                    Some(s) => temp.push(s),
                    None => {
                        std::thread::yield_now();
                        continue;
                    }
                }
            }
            buffer.copy_from_slice(&temp[..WINDOW]);
            
            // process FFT
            
            for i in 0..27 {
                leds[i] = LEDRecord {
                    index: i,
                    color: (0u8, 0u8, 255u8),
                };
            }
            write_frame(&mut port, &leds).unwrap();

            temp.drain(..HOP);
        }
    });

    std::thread::park();
}