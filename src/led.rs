#![allow(dead_code)]

use crate::utils;

pub fn led_on(port: u32, pin: u32)
{
    let gpio_bsrr = (port + 0x10) as *mut u32; // BSRR
    unsafe
    {
        utils::write_register(gpio_bsrr, 1 << pin);
    }
}

pub fn led_off(port: u32, pin: u32)
{
    let gpio_bsrr = (port + 0x10) as *mut u32; // BSRR
    unsafe
    {
        utils::write_register(gpio_bsrr, 1 << (pin + 16)); // reset bit
    }
}

pub fn led_toggle(port: u32, pin: u32)
{
    let gpio_odr = (port + 0x0C) as *mut u32; // ODR
    unsafe
    {
        utils::toggle_register(gpio_odr, 1 << pin);
    }
}
