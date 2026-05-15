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
use crate::nb::block;

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
    #[allow(deprecated)]
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

    fn log(message: &str, uart: &mut impl _embedded_hal_serial_Write<u8>) {
        for byte in message.bytes() {
            let _ = block!(uart.write(byte));
        }
    }

    loop {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        log("Hello, world!\n", &mut uart);
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
