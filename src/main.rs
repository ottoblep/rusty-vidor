#![no_std]
#![no_main]

use arduino_mkrvidor4000::clock::{ClockGenId, ClockSource};
use arduino_mkrvidor4000 as bsp;
use bsp::hal;

use panic_halt as _;
use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::sercom::v2::{Sercom5, uart};
use hal::sercom::v2::uart::{BaudMode, BitOrder, NineBit, Oversampling, Pads, StopBits};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = bsp::Pins::new(peripherals.PORT);

    // UART Clock
    clocks.configure_gclk_divider_and_source(ClockGenId::GCLK0, 1, ClockSource::XOSC32K, false);
    clocks.configure_standby(ClockGenId::GCLK0, true);
    let gclk = clocks.get_gclk(ClockGenId::GCLK0).unwrap();
    clocks.sercom5_core(&gclk);

    // GPIO
    let mut led = pins.led_builtin.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // UART
    let pads = Pads::<Sercom5>::default()
        .rx(pins.rx)
        .tx(pins.tx);
    let mut uart = uart::Config::new(&(peripherals.PM), peripherals.SERCOM5, pads, 48.mhz())
        .baud(115200.hz(), BaudMode::Arithmetic(Oversampling::Bits8))
        .char_size::<NineBit>()
        .bit_order(BitOrder::LsbFirst)
        .stop_bits(StopBits::OneBit)
        .enable();


    loop {
        delay.delay_ms(200u8);
        let data: u16 = 0xABCD;
        let _ = uart.write(data);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
