use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;
use ringbuf::HeapRb;
use ringbuf::traits::Split;
use ringbuf::traits::{Producer, Consumer};
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};
use rustfft::num_traits::Zero;
use std::f32::consts::PI;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
struct LEDRecord {
    index: usize,
    color: (u8, u8, u8),
}

const WINDOW: usize = 2048;
const HOP: usize = 512;

const NEIGHBORS: [&[usize]; 27] = [
    &[1,6], &[0,2,7], &[1,3,8], &[2,4,9], &[3,5,10], &[4,11],
    &[0,7,12], &[1,6,8,13], &[2,7,9,14], &[3,8,10,15], &[4,9,11,16], &[5,10,17],
    &[6,13,18], &[7,12,14,19], &[8,13,15,20], &[9,14,16,21], &[10,15,17,22], &[11,16,23],
    &[12,19], &[13,18,20], &[14,19,21], &[15,20,22], &[16,21,23], &[17,22],
    &[18], &[19], &[20],
];

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as i32 {
        h if h < 60 => (c, x, 0.0),
        h if h < 120 => (x, c, 0.0),
        h if h < 180 => (0.0, c, x),
        h if h < 240 => (0.0, x, c),
        h if h < 300 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
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
    let fft = FftPlanner::new().plan_fft_forward(WINDOW);
    let mut input: Vec<Complex<f32>> = vec![Complex::zero(); WINDOW];
    let hann: Vec<f32> = (0..WINDOW)
        .map(|i| 0.5 - 0.5 * f32::cos(2.0 * PI * i as f32 / WINDOW as f32))
        .collect();

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
        let mut smooth: [f32; 27] = [0.0; 27];
        let mut temp = Vec::with_capacity(WINDOW);
        let mut heights_smooth = [0f32; 6];
        let mut energy = [0f32; 27];
        let mut prev_bass = 0.0;
        let mut max_energy = 1e-6f32;

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

            for i in 0..WINDOW {
                input[i].re = buffer[i] * hann[i];
                input[i].im = 0.0;
            }

            fft.process(&mut input);

            let magnitudes: Vec<f32> = input[..WINDOW / 2]
                .iter()
                .map(|c| (c.re * c.re + c.im * c.im).sqrt())
                .collect();
            
            let min_freq = 40.0;
            let max_freq = sample_rate as f32 / 2.0;

            let mut bands = [0f32; 27];

            for (i, mag) in magnitudes.iter().enumerate() {
                let freq = i as f32 * sample_rate as f32 / WINDOW as f32;

                if freq < min_freq || freq > max_freq { continue; }

                let norm = ((freq / min_freq).ln() / (max_freq / min_freq).ln()).powf(0.5);
                let band = (norm * 27.0) as usize;

                if band < 27 {
                    bands[band] += mag;
                }
            }

            for i in 0..27 {
                let weight = (i as f32 / 27.0).powf(0.5);
                bands[i] *= weight;
            }

            let bass = bands[0] + bands[1] + bands[2];
            let mids = bands[5] + bands[6] + bands[7];
            let highs = bands[15] + bands[16] + bands[17];

            if bass > 0.01 {
                let center = 13;
                energy[center] += bass * 10.0;

                for &n in NEIGHBORS[center] {
                    energy[n] += bass * 5.0;
                }
            }

            if highs > 0.005 {  
                let idx = rand::random_range(0..27);
                energy[idx] += highs * 3.0;
            }

            let mut new_energy = [0.0f32; 27];

            for i in 0..27 {
                new_energy[i] += energy[i] * 0.6;

                for &n in NEIGHBORS[i] {
                    new_energy[n] += energy[i] * 0.1;
                }
            }

            for i in 0..27 {
                new_energy[i] *= 0.6; // decay
            }

            energy = new_energy;


            let t = std::time::Instant::now().elapsed().as_millis() as f32 * 0.01;

            for i in 0..27 {
                let wave = ((i as f32 * 0.5 + t).sin() * 0.5 + 0.5);
                let audio = energy[i].min(1.0);

                let e = wave * (0.3 + audio * 0.7);

                let hue = (200.0 - audio * 200.0) + (wave * 40.0);

                let (r, g, b) = hsv_to_rgb(hue, 1.0, e);

                leds[i] = LEDRecord {
                    index: i,
                    color: (r, g, b),
                };
            }

            write_frame(&mut port, &leds).unwrap();

            temp.drain(..HOP);
        }
    });

    std::thread::park();
}