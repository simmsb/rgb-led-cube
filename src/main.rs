#![no_std]
#![no_main]

use defmt::info;

use hal::prelude::*;
use hal::spi::Spi;
use hal::{gpio::GpioExt, prelude::_stm32_hal_flash_FlashExt, rcc::RccExt};
use led_test as _;

use led_test::dither::GammaDither;
use smart_leds::RGB8;
use stm32f1xx_hal as hal;

use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};

fn into_grb(it: impl Iterator<Item = RGB8>) -> impl Iterator<Item = RGB8> {
    it.map(|a| RGB8::new(a.g, a.r, a.b))
}

fn conv_colour(c: cichlid::ColorRGB) -> RGB8 {
    RGB8::new(c.r, c.g, c.b)
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello world!");

    let dp = hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(72.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    let pins = (
        gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
        gpiob.pb14.into_floating_input(&mut gpiob.crh),
        gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
    );
    let mut delay = dp.TIM1.delay::<1_000_000>(&clocks);

    let spi = Spi::spi2(dp.SPI2, pins, ws2812_spi::MODE, 3_800.kHz(), clocks);

    let mut leds = ws2812_spi::Ws2812::new(spi);

    let mut i = 0u8;

    const STEPS: usize = 16;

    loop {
        for step in 0..STEPS {
            let it = (0..10).map(|x| {
                let v = cichlid::HSV {
                    h: i.wrapping_add(x * 10),
                    s: 255,
                    v: 127,
                };
                conv_colour(v.to_rgb_rainbow())
            });

            let _ = leds.write(into_grb(GammaDither::<STEPS, 28>::dither(step, it)));

            delay.delay_us(500u32);
        }
        i = i.wrapping_add(1);
    }
}
