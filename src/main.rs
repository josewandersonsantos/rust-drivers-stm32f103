// https://github.com/niekiran/embedded-rust
// cargo flash --chip STM32F103C8T6
#![no_std]
#![no_main]

use core::{panic::PanicInfo};

mod startup_stm32f103;
mod utils;
mod mcu;
mod rcc;
mod irq;
mod gpio;
mod led;
mod i2c;
mod usart;
/*
 * PANIC HANDLER
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}

/*
 * MAIN
 */
#[no_mangle]
fn main() -> !
{
    // RCC (GPIOC)
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPc);    
    // PC13 (LED)
    gpio::configure_pin(mcu::GPIOC_BASE, mcu::GPIO13, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));

    loop
    {
        // Toggle LED on PC13
        led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
        // Delay 1s
        utils::delay_ms(1000);
    }
}