# STM32F103 Rust Drivers

![Status](https://img.shields.io/badge/status-in%20progress-yellow)
![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![Platform](https://img.shields.io/badge/platform-STM32F103-03234B?logo=stmicroelectronics)
![Architecture](https://img.shields.io/badge/architecture-ARM_Cortex--M3-0091BD?logo=arm)
![Environment](https://img.shields.io/badge/environment-no__std-success)
![License](https://img.shields.io/github/license/josewandersonsantos/rust-drivers-stm32f103)

A lightweight, `no_std` embedded Rust library for the **STM32F103 (Blue Pill)**, providing low-level drivers for core peripherals such as GPIO, RCC, I2C, USART, and more.

This project focuses on simplicity, control, and transparency, making it ideal for learning, experimentation, and building custom embedded systems from scratch.

---

## ✨ Features

* `no_std` compatible (bare-metal support)
* Direct register-level access (no heavy abstractions)
* Modular peripheral drivers:
  * GPIO
  * RCC (clock configuration)
  * I2C
  * USART
  * ...

* Minimal dependencies
* Designed for STM32F103 (Cortex-M3)

---

## 🎯 Goals

* Provide a clean and understandable low-level interface
* Avoid unnecessary abstractions
* Serve as a learning tool for embedded Rust and ARM Cortex-M
* Allow full control over hardware behavior

---

## 📦 Project Structure

```
src/
├── gpio.rs
├── i2c.rs
├── irq.rs
├── mcu.rs
├── rcc.rs
├── usart.rs
├── utils.rs
└── main.rs / lib.rs
```

---

## 🚀 Getting Started

### Requirements

* Rust toolchain
* Target installed:

```
rustup target add thumbv7m-none-eabi
```

* Probe tool:

```
cargo install probe-rs-tools
```

---

### Build

```
cargo build --target thumbv7m-none-eabi
```

---

### Run (example using probe-rs)

```
cargo run --target thumbv7m-none-eabi
```

---

## ⚙️ Configuration

Example `.cargo/config.toml`:

```toml
[build]
target = "thumbv7m-none-eabi"

[target.thumbv7m-none-eabi]
rustflags = ["-C", "link-arg=-Tmemory.ld"]
runner = "probe-rs run --chip STM32F103C8T6"
```

---

## 🧠 Design Philosophy

This library intentionally avoids using high-level HALs (like `stm32f1xx-hal`) in favor of:

* Explicit register manipulation
* Full hardware control
* Better understanding of the MCU internals

---

## 📡 Example Usage

```rust
// Example (pseudo-code)

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
```

---

## ⚠️ Disclaimer

This is a low-level implementation. It assumes:

* Knowledge of embedded systems
* Familiarity with STM32 reference manuals
* Understanding of unsafe Rust

---

## 📚 References

* STM32F103 Reference Manual
* ARM Cortex-M3 Documentation

---

## 🚧 Roadmap

* [X] RCC driver
* [X] GPIO driver
* [X] I2C driver
* [X] USART driver
* [ ] SPI driver
* [ ] CRC driver
* [ ] USB driver
* [ ] Timer (PWM / delays)
* [X] Interrupt support
* [ ] WWDG
* [ ] DMA
* [ ] Better error handling
* [ ] Optional safe abstractions layer

---

## 🤝 Contributing

Contributions are welcome! Feel free to:

* Open issues
* Suggest improvements
* Submit pull requests

---

## 📄 License

Apache 2.0

---

## 💡 Author Notes

This project is part of a deeper exploration into:

* Embedded Rust
* Bare-metal programming
* Sensor integration (IMU, GNSS)
* Real-time systems

---

Enjoy hacking at the metal 🚀
