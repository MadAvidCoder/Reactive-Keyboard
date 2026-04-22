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

    fn fill_color(&mut self, r: u8, g: u8, b: u8) {
        for i in 0..LEDS {
            Self::encode_color(r, g, b, &mut self.buf, i * BITS_PER_LED);
        }

        for i in (LEDS * BITS_PER_LED)..self.buf.len() {
            self.buf[i] = 0;
        }
    }

    async fn write(&mut self) {

        let seq_cfg = SequenceConfig::default();
        let seq = Sequence::new(&self.buf, seq_cfg);

        let seqr = Sequencer::new(&mut self.pwm, seq, None);

        seqr.start(StartSequence::Zero, SequenceMode::Loop(1)).unwrap();
        Timer::after_micros(100).await;
    }

    pub async fn run(&mut self) -> ! {
        loop {
            // Green
            self.fill_color(0, 255, 0);
            self.write().await;
            Timer::after(Duration::from_millis(500)).await;

            // Red
            self.fill_color(255, 0, 0);
            self.write().await;
            Timer::after(Duration::from_millis(500)).await;

            // Blue
            self.fill_color(0, 0, 255);
            self.write().await;
            Timer::after(Duration::from_millis(500)).await;

            // Off
            self.fill_color(0, 0, 0);
            self.write().await;
            Timer::after(Duration::from_millis(500)).await;
        }
    }
}