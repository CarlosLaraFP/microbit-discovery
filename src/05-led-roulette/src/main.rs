#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut led_display = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let roulette = [
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 4), (2, 4), (3, 4), (4, 4),
        (4, 3), (4, 2), (4, 1), (4, 0),
        (3, 0), (2, 0), (1, 0)
    ];

    loop {
        for (i, j) in roulette {
            let mut copy = led_display.clone();
            copy[i][j] = 1;
            display.show(&mut timer, copy, 250);
        }
    }
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