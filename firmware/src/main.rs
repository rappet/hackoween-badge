#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use ch32_hal as hal;
use ch32_hal::gpio::Speed;
use hal::delay::Delay;
use hal::gpio::{Level, Output};
use hal::println;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_delay::Ws2812;

const COLOR_WHEEL: [[u8; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

const MAX_VALUE: usize = 32;

const WHEEL_MODULO: usize = 3 * MAX_VALUE * 2;

fn mix(i: usize) -> [u8; 3] {
    // hardest problems in programming: naming things

    let color_id = i / (MAX_VALUE * 2);
    let mult_id = i % (MAX_VALUE * 2);

    let color_a: [u8; 3] = COLOR_WHEEL[color_id];
    let color_b: [u8; 3] = COLOR_WHEEL[(color_id + 1) % 3];
    let mult_a = if mult_id >= MAX_VALUE {
        (MAX_VALUE * 2) - (mult_id)
    } else {
        MAX_VALUE
    } as u8;
    let mult_b = if mult_id >= MAX_VALUE {
        MAX_VALUE
    } else {
        mult_id
    } as u8;

    [
        color_a[0] * mult_a + color_b[0] * mult_b,
        color_a[1] * mult_a + color_b[1] * mult_b,
        color_a[2] * mult_a + color_b[2] * mult_b,
    ]
}

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let p = hal::init(config);

    let mut delay = Delay;

    let smartled_pin = Output::new(p.PA1, Level::Low, Speed::High);

    let mut smartled = Ws2812::new(Delay, smartled_pin);

    let mut colors = [RGB8 { r: 0, g: 0, b: 0 }; 2];

    for _ in 0..100 {
        smartled.write(colors.into_iter());
        Delay.delay_ms(10);
    }

    colors = [RGB8 {
        r: 63,
        g: 63,
        b: 63,
    }; 2];
    smartled.write(colors.into_iter());
    Delay.delay_ms(100);

    let mut iter: usize = 0;

    loop {
        iter = iter.wrapping_add(1);
        //led.toggle();

        for (i, color) in colors.iter_mut().enumerate() {
            let idx =
                i.wrapping_add(i.wrapping_mul(128)).wrapping_add(iter) % (WHEEL_MODULO as usize);
            let c = mix(idx);
            *color = RGB8 {
                r: c[0],
                g: c[1],
                b: c[2],
            };
        }

        smartled.write(colors.into_iter());

        delay.delay_ms(10);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = println!("\n\n\n{}", info);

    loop {}
}
