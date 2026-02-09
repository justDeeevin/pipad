use embassy_stm32::{adc::SampleTime, gpio::Speed};
use keyberon::key_code::KeyCode;

pub const PRESS_DELTA_THRESHOLD: i16 = -3;
pub const RELEASE_DELTA_THRESHOLD: i16 = -1;

pub const SELECT_SPEED: Speed = Speed::Medium;
pub const ADC_CYCLES: SampleTime = SampleTime::CYCLES15;

pub const KEYPRESS_FILTER_SHIFT: i16 = 2;

pub const AM0_KEYCODES: [Option<KeyCode>; 16] = {
    use KeyCode::*;
    [
        Some(Kp0),
        Some(Kp1),
        Some(Kp4),
        None,
        None,
        None,
        Some(Kp7),
        Some(NumLock),
        Some(KpSlash),
        Some(Kp8),
        None,
        None,
        None,
        None,
        Some(Kp5),
        Some(Kp2),
    ]
};

pub const AM1_KEYCODES: [Option<KeyCode>; 16] = {
    use KeyCode::*;
    [
        Some(KpDot),
        Some(Kp3),
        Some(Kp6),
        None,
        None,
        None,
        Some(Kp9),
        Some(KpAsterisk),
        Some(KpMinus),
        None,
        None,
        None,
        None,
        None,
        Some(KpPlus),
        Some(KpEnter),
    ]
};
