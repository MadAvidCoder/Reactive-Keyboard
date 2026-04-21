// use embassy_nrf::pwm::{Sequence, SequenceConfig, SingleSequenceMode};
// use smart_leds::{SmartLedsWrite, RGB8};
// use embassy_time::Timer;
//
// pub struct RgbTask<'a> {
//     pwm: embassy_nrf::pwm::SequencePwm<'a>,
//     buf: [u16; 24 * 8],
// }
//
// impl <'a> RgbTask<'a> {
//     pub fn new(pwm: embassy_nrf::pwm::SequencePwm<'a>) -> Self {
//         Self {
//             pwm,
//              buf: [0; 24 * 8],
//         }
//     }
//
//     fn encode_byte(byte: u8, out: &mut [u16], mut idx: usize) -> usize {
//         for i in (0..8).rev(){
//             let bit = (byte >> i) & 1;
//             out[idx] = if bit == 1 {14} else {6};
//             idx += 1;
//         }
//         idx
//     }
//
//     fn encode_color(&mut self, color: RGB8) {
//         let mut idx = 0;
//         idx = Self::encode_byte(color.g, &mut self.buf, idx);
//         idx = Self::encode_byte(color.r, &mut self.buf, idx);
//         idx = Self::encode_byte(color.b, &mut self.buf, idx);
//     }
//
//     pub async fn run(&mut self) -> ! {
//         loop {
//             self.encode_color(RGB8 { r:0, g: 0, b: 50});
//             let mut s_cfg = SequenceConfig::default();
//             s_cfg.refresh = 0;
//             s_cfg.end_delay = 0;
//
//             // let seq = Sequence::new(&self.buf, s_cfg);
//             let _ = self.pwm.sequence(0, &seq, SingleSequenceMode::Times(1));
//             Timer::after_millis(20).await;
//         }
//     }
// }

use embassy_nrf::pwm::DutyCycle;
use embassy_time::Timer;

pub struct RgbTask<'a> {
    pwm: embassy_nrf::pwm::SimplePwm<'a>,
}

impl<'a> RgbTask<'a> {
    pub fn new(pwm: embassy_nrf::pwm::SimplePwm<'a>) -> Self {
        Self { pwm }
    }

    pub async fn run(&mut self) -> ! {
        loop {
            // crude "on" signal test
            let max = 20;
            let duty = 10;

            self.pwm.set_duty(0, DutyCycle::normal(duty));
            Timer::after_millis(100).await;
        }
    }
}