# Hackoween badge

This has been made with a very hot needle in a very short timeframe.

PCB design is suboptimal.

It is based on the [WCH CH32V003](https://www.wch-ic.com/products/CH32V003.html)
MCU and draws between 11mA and 14mA at 5V.

![the badge](img/badge.jpg)

## Programming

To flash the badge, you need to have a [WCH-LinkE programmer](
https://www.wch-ic.com/products/WCH-Link.html) and the
[`wlink` CLI tool](https://github.com/ch32-rs/wlink).

NOTE: The older WCH-Link (without E) does _not_ work!

Connect as follows:

- `TP3` is the _SWDIO_ pin
- `TP1` is 5V VCC
- `TP2` is GND (ground)

In the `firmware/` directory, run:
```sh
cargo run --release
```
