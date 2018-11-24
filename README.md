# ssd1327-rs
This repository contains code to communicate with i2c ssd1327 controlled grey scale oled displays with the [Rust](https://www.rust-lang.org/en-US/) language. It works on top of [i2cdev](https://github.com/rust-embedded/rust-i2cdev) and is intended to work on the raspberry pi.

It is loosely based on the ssd1306 mono-display driver, from https://github.com/wheelin/ssd1306-rs.

With 8 bit font support from: https://github.com/saibatizoku/font8x8-rs