#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![allow(clippy::ignored_unit_patterns)]

use command_rs as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = microbit::pac,
    dispatchers = [SWI0_EGU0]
)]
mod app {
    use embedded_hal::serial::Read;
    use microbit::{
        hal::{
            uarte::{Baudrate, Parity, Pins, UarteRx, UarteTx},
            Uarte,
        },
        pac::UARTE0,
    };
    use rtic_monotonics::nrf::rtc::{ExtU64, Rtc0};

    const SERIAL_RX_BUF_SIZE: usize = 1;
    const SERIAL_TX_BUF_SIZE: usize = 256;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        _tx: UarteTx<UARTE0>,
        rx: UarteRx<UARTE0>,
    }

    #[init(local = [ tx_buf: [u8; SERIAL_TX_BUF_SIZE] = [0u8; SERIAL_TX_BUF_SIZE], rx_buf: [u8; SERIAL_RX_BUF_SIZE] = [0u8; SERIAL_RX_BUF_SIZE] ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let board = microbit::Board::new(cx.device, cx.core);

        let uart_pins = Pins::from(board.uart);

        let rx_buf = cx.local.rx_buf;

        let (tx, rx) = Uarte::new(
            board.UARTE0,
            uart_pins,
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
        .split(&mut cx.local.tx_buf[..], rx_buf)
        .unwrap();

        let token = rtic_monotonics::create_nrf_rtc0_monotonic_token!();
        Rtc0::start(board.RTC0, token);

        command_client::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local { _tx: tx, rx },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        #[allow(clippy::empty_loop)]
        loop {}
    }

    #[task(priority = 1, local = [ rx ])]
    async fn command_client(cx: command_client::Context) {
        defmt::trace!("Starting client of command pattern.");

        let rx = cx.local.rx;

        loop {
            // Let the scheduler switch to a different task before reading (again)
            Rtc0::delay(5.millis()).await;

            match rx.read() {
                Ok(byte) => defmt::trace!("Got {}", char::try_from(byte)),
                // Err(nb::Error::WouldBlock) => continue,
                _ => continue,
            }
        }
    }
}
