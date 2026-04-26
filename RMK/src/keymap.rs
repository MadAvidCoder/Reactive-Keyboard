use rmk::types::action::KeyAction::{self};
use rmk::{a, k, layer, mo};

pub(crate) const COL: usize = 12;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 2;
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Minus), k!(Equal)],
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(LeftBracket), k!(RightBracket), k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [k!(A), k!(S), k!(D), k!(F), k!(G), k!(Backslash), k!(Slash), k!(H), k!(J), k!(K), k!(L), k!(Quote)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B), k!(Comma), k!(Dot), k!(N), k!(M), k!(Semicolon), k!(Quote), k!(Grave)],
            [a!(No), a!(No), a!(No), k!(Tab), k!(LShift), k!(Space), mo!(1), k!(Backspace), k!(Delete), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Minus), k!(Equal)],
            [a!(No), k!(Up), a!(No), a!(No), k!(Home), k!(PageUp), k!(PageDown), k!(End), k!(Up), a!(No), a!(No), a!(No)],
            [k!(Left), k!(Down), k!(Right), k!(LShift), a!(No), a!(No), a!(No), k!(Left), k!(Down), k!(Right), a!(No), a!(No)],
            [k!(LCtrl), k!(LAlt), k!(LGui), k!(LShift), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(LAlt), k!(LGui), k!(LCtrl), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
    ]
}