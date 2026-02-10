#![no_std]
#![no_main]

mod consts;

use consts::*;

use defmt::{Debug2Format, debug, info, trace};
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, adc::AdcChannel, gpio::Speed, peripherals::ADC1, rcc::{Hse, HseMode, Sysclk}, time::Hertz
};
use embassy_stm32::{
    Peri,
    adc::{Adc, AdcConfig, Resolution},
    gpio::{Level, Output, Pin},
};
use keyberon::key_code::KeyCode;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = {
        let mut config = Config::default();

        // 26 MHz (max freq) crystal oscillator sysclock
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(26),
            mode: HseMode::Oscillator,
        });
        config.rcc.sys = Sysclk::HSE;

        embassy_stm32::init(config)
    };

    info!("pipad firmware v{}", env!("CARGO_PKG_VERSION"));

    // High = lit, Low = unlit
    // I hate it :<
    let mut led = Output::new(board.PC13, Level::High, Speed::Low);
    let mut am0 = board.PA1;
    let mut am1 = board.PA0;

    let mut adc = Adc::new_with_config(
        board.ADC1,
        AdcConfig {
            resolution: Some(Resolution::BITS12),
        },
    );
    let mut mux = {
        fn pin(pin: Peri<'static, impl Pin>) -> Output<'static> {
            Output::new(pin, Level::Low, SELECT_SPEED)
        }

        Mux::new([
            pin(board.PC14),
            pin(board.PB8),
            pin(board.PB12),
            pin(board.PA15),
        ])
    };

    let mut i = 0;
    let mut am0_keys = AM0_KEYCODES.map(|code| {
        let Some(code) = code else {
            i += 1;
            return None;
        };

        mux.select(i);

        let value = (0..8).map(|_| read(&mut adc, &mut am0)).sum::<Value>() / 8;
        debug!("am0:{} ({}) rests at {}", i, Debug2Format(&code), value);

        let out = Some(KeyState::new(code, value));

        i += 1;

        out
    });
    i = 0;
    let mut am1_keys = AM1_KEYCODES.map(|code| {
        let Some(code) = code else {
            i += 1;
            return None;
        };

        mux.select(i);

        let value = (0..8).map(|_| read(&mut adc, &mut am1)).sum::<Value>() / 8;
        debug!("am1:{} ({}) rests at {}", i, Debug2Format(&code), value);

        let out = Some(KeyState::new(code, value));
        i += 1;
        out
    });

    info!("initialized");

    loop {
        for (i, (key_0, key_1)) in am0_keys
            .iter_mut()
            .zip(&mut am1_keys)
            .map(|(a, b)| (a.as_mut(), b.as_mut()))
            .enumerate()
        {
            let mut any_pressed = false;
            let mut selected = false;

            if let Some(key_0) = key_0 {
                mux.select(i as u8);
                selected = true;

                key_0.update(read(&mut adc, &mut am0));

                if !any_pressed && key_0.pressed {
                    any_pressed = true;
                }
            }

            if let Some(key_1) = key_1 {
                if !selected {
                    mux.select(i as u8);
                }

                key_1.update(read(&mut adc, &mut am1));

                if !any_pressed && key_1.pressed {
                    any_pressed = true;
                }
            }

            if any_pressed != led.is_set_low() {
                led.set_level((!any_pressed).into());
            }
        }
    }
}

struct Mux {
    select: [Output<'static>; 4],
}

impl Mux {
    fn new(select: [Output<'static>; 4]) -> Self {
        Self { select }
    }

    fn select(&mut self, index: u8) {
        assert!(index < 16, "invalid multiplexer index");

        trace!("select {}", index);

        for (i, select) in self.select.iter_mut().enumerate() {
            select.set_level((index >> i & 1 == 1).into());
        }
    }
}

struct KeyState {
    resting: Value,
    filtered: Value,
    pressed: bool,
    code: KeyCode,
}

impl KeyState {
    fn new(code: KeyCode, resting_raw: Value) -> Self {
        let filtered = filtered_resting(resting_raw);
        Self {
            resting: filtered,
            filtered,
            pressed: false,
            code,
        }
    }

    // TODO: send keycodes
    fn update(&mut self, raw: Value) {
        self.filtered += (raw - self.filtered) >> KEYPRESS_FILTER_SHIFT;
        trace!(
            "raw {}, filtered {} on {}",
            raw,
            self.filtered,
            Debug2Format(&self.code)
        );

        let delta = self.filtered - self.resting;

        if !self.pressed && delta < PRESS_DELTA_THRESHOLD {
            debug!("{} pressed", Debug2Format(&self.code));
            self.pressed = true;
        } else if self.pressed && delta > RELEASE_DELTA_THRESHOLD {
            debug!("{} released", Debug2Format(&self.code));
            self.pressed = false;
        }
    }
}

fn read(adc: &mut Adc<ADC1>, pin: &mut impl AdcChannel<ADC1>) -> Value {
    let _ = adc.blocking_read(pin, ADC_CYCLES);
    adc.blocking_read(pin, ADC_CYCLES) as Value
}
