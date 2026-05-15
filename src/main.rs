#![no_std]
#![no_main]

use arduino_mkrvidor4000 as bsp;
use bsp::hal;

use bsp::entry;
use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use panic_halt as _;

use hal::sercom::v2::uart::{BaudMode, BitOrder, EightBit, Oversampling, Pads, StopBits};
use hal::sercom::v2::{Sercom5, uart};

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
    let gclk = clocks.get_gclk(ClockGenId::GCLK0).unwrap();
    clocks.sercom5_core(&gclk);

    // GPIO
    let mut led = pins.led_builtin.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // UART
    let pads = Pads::<Sercom5>::default().rx(pins.rx).tx(pins.tx);
    let mut uart = uart::Config::new(&(peripherals.PM), peripherals.SERCOM5, pads, 48.mhz())
        .baud(115200.hz(), BaudMode::Arithmetic(Oversampling::Bits8))
        .char_size::<EightBit>()
        .bit_order(BitOrder::LsbFirst)
        .stop_bits(StopBits::OneBit)
        .parity(uart::Parity::None)
        .enable();

    loop {
        delay.delay_ms(200u8);
        let data: char = 'A';
        let _ = uart.write(data as u8);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
