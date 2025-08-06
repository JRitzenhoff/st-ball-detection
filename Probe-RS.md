# Probe-rs

This is a tool that communicates to the ST device over ST-Link

## Finicky-ness with the STM32 board

1. If the board isn't showing up, try communicating to it using the STM32-Programmer
    * This can put it into a state that it's happy with
    * I did a firmware upgrade and that appeared to help (even though the firmware version wasn't changing)
1. `probe-rs list` should show the device as an ST-Link
1. `prove-rs info` should show the device's information
    * _NOTE_: If this doesn't work, try adding the `--connect-under-reset` flag
    * Because the board being used is ultra-low power, I guess it has to be connected to while reset

## Programming with probe-rs
1. If a program is active, need to program using `--connect-under-reset`
    * `cargo run --bin blinky -- --connect-under-reset`