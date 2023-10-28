#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use command_rs as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = microbit::pac,
    dispatchers = [SWI0_EGU0]
)]
mod app {
    use microbit::{
        hal::{
            uarte::{Baudrate, Parity, Pins},
            Uarte,
        },
        pac::UARTE0,
    };

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        serial: Uarte<UARTE0>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let board = microbit::Board::new(cx.device, cx.core);

        let uart_pins = Pins::from(board.uart);

        let uarte0 = Uarte::new(
            board.UARTE0,
            uart_pins,
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );

        // TODO setup monotonic if used
        // let sysclk = { /* clock setup + returning sysclk as an u32 */ };
        // let token = rtic_monotonics::create_systick_token!();
        // rtic_monotonics::systick::Systick::new(cx.core.SYST, sysclk, token);

        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local { serial: uarte0 },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // TODO: Add tasks
    #[task(priority = 1, local = [ serial ])]
    async fn task1(cx: task1::Context) {
        defmt::info!("Hello from task1!");

        // Force msg to be located in RAM
        let mut msg = [0u8; 16];
        msg.copy_from_slice(b"Hello via UART\r\n");

        let serial = cx.local.serial;

        let _ = serial.write(&msg).unwrap();
    }
}
