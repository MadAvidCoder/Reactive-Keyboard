use embassy_nrf::pwm::{Sequencer, StartSequence, SequenceMode, Sequence, SequencePwm, Config as PwmConfig, SequenceConfig};
use embassy_time::{Duration, Timer};

const LEDS: usize = 8;
const BITS_PER_LED: usize = 24;
const RESET_SLOTS: usize = 60;

const DUTY_0: u16 = 5;
const DUTY_1: u16 = 15;
const PWM_TOP: u16 = 20;

pub struct RgbTask<'d> {
    pwm: SequencePwm<'d>,
    buf: [u16; LEDS * BITS_PER_LED + RESET_SLOTS],
}

impl<'d> RgbTask<'d> {
    pub fn new(pwm: SequencePwm<'d>) -> Self {
        Self {
            pwm,
            buf: [0; LEDS * BITS_PER_LED + RESET_SLOTS],
        }
    }

    fn encode_byte(byte: u8, out: &mut [u16], offset: usize) {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            out[offset + i] = if bit == 1 { 0x8000 | DUTY_1 } else { 0x8000 | DUTY_0 };
        }
    }

    fn encode_color(r: u8, g: u8, b: u8, out: &mut [u16], offset: usize) {
        Self::encode_byte(g, out, offset);
        Self::encode_byte(r, out, offset + 8);
        Self::encode_byte(b, out, offset + 16);
    }

    fn set_pixel(&mut self, pix: usize, r: u8, g: u8, b: u8) {
        // pix = 0 enc
        Self::encode_color(r, g, b, &mut self.buf, pix * BITS_PER_LED);
    }

    fn fill_color(&mut self, r: u8, g: u8, b: u8) {
        for i in 0..LEDS {
            self.set_pixel(i, r, g, b)
        }

        for i in (LEDS * BITS_PER_LED)..self.buf.len() {
            self.buf[i] = 0;
        }
    }

    fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
        if s == 0 {
            return (v, v, v);
        }

        let region = h / 43;
        let remainder = (h as u16 - (region as u16) * 43) * 6;

        let p = (v as u16 * (255 - s as u16)) / 255;
        let q = (v as u16 * (255 * 43 - s as u16 * remainder as u16) / 43) / 255;
        let t = (v as u16 * (255 * 43 - s as u16 * (255 * 43 - remainder as u16) / 43)) / 255;

        let (r, g, b) = match region {
            0 => (v, t as u8, p as u8),
            1 => (q as u8, v, p as u8),
            2 => (p as u8, v, t as u8),
            3 => (p as u8, q as u8, v),
            4 => (t as u8, p as u8, v),
            _ => (v, p as u8, q as u8),
        };

        (r, g, b)
    }

    async fn write(&mut self) {
        let seq_cfg = SequenceConfig::default();
        let seq = Sequence::new(&self.buf, seq_cfg);

        let seqr = Sequencer::new(&mut self.pwm, seq, None);

        seqr.start(StartSequence::Zero, SequenceMode::Loop(1)).unwrap();
        Timer::after_micros(200).await;
    }

    pub async fn run(&mut self) -> ! {
        let mut frame: u8 = 0;

        loop {
            for i in 0..LEDS {
                let hue = frame.wrapping_add((i as u8) * 32);

                let (r, g, b) = Self::hsv_to_rgb(hue, 255, 255);
                self.set_pixel(i, r, g, b);
            }

            for i in (LEDS * BITS_PER_LED)..self.buf.len() {
                self.buf[i] = 0;
            }

            self.write().await;

            frame = frame.wrapping_add(1);

            Timer::after(Duration::from_millis(500)).await;
        }
    }
}