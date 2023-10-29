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
    use rtic_monotonics::nrf::rtc::{ExtU64, Rtc0};

    const COMMAND_BUFFER_SIZE: usize = 8;

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

        let token = rtic_monotonics::create_nrf_rtc0_monotonic_token!();
        Rtc0::start(board.RTC0, token);

        command_client::spawn().ok();

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

    #[task(priority = 1, local = [ serial ])]
    async fn command_client(cx: command_client::Context) {
        defmt::trace!("Starting client of command pattern.");

        let mut buf = [0u8; COMMAND_BUFFER_SIZE];
        let serial = cx.local.serial;

        loop {
            // Read COMMAND_BUFFER_SIZE bytes from serial blockingly
            let bytes = match serial.read(&mut buf) {
                // let bytes = match serial.read_timeout(&mut buf, timer, 1_000) {
                Ok(()) => buf.len(),
                _ => 0,
            };

            if bytes > 0 {
                defmt::trace!("Got '{}'", buf[..bytes]);
                let _ = serial.write(&buf[..bytes]).unwrap();
            }

            // Let the scheduler switch to a different task before reading again
            Rtc0::delay(5.millis()).await
        }
    }
}
