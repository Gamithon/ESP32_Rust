#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::{ClockControl, self}, peripherals::Peripherals, prelude::*, Delay, IO, system};
//use ws2812_esp32_rmt_driver as LED_driver;

#[entry]
fn main() -> ! {
    //Hz = NOM / DENOM * raw    
    //let processer_speed = ClockControl::max(system.clock_control).freeze().cpu_clock;

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();    
    let mut delay = Delay::new(&clocks);
    
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio13.into_push_pull_output();

    led.set_high().unwrap();

    loop {
        led.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}
