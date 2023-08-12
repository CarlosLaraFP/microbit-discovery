#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}

// ** builds and flashes in 1 command **
// cargo embed --features v2 --target thumbv7em-none-eabihf

/*
    gdb target/thumbv7em-none-eabihf/debug/led-roulette
    target remote :1337
    break main (or next)
    continue
    info break
    info locals
    layout asm -> layout src
    monitor reset
    c
 */
// lldb target/thumbv7em-none-eabihf/debug/led-roulette