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
    use core::{convert::TryFrom, fmt::Write};
    use embedded_hal::serial::{Read, Write as EmbeddedWrite};
    use heapless::Vec;
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

    const COMMAND_BUF_SIZE: usize = 32;

    enum Command {
        Help,
    }

    impl Command {
        fn execute(self) {
            match self {
                Command::Help => command_help::spawn().unwrap(),
            }
        }
    }

    impl TryFrom<&[u8]> for Command {
        type Error = &'static str;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            match value {
                b"help" => Ok(Command::Help),
                _ => Err("not a command"),
            }
        }
    }

    #[shared]
    struct Shared {
        // Better make this local to one sender task which gets its workload from a queue
        tx: UarteTx<UARTE0>,
    }

    #[local]
    struct Local {
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

        (Shared { tx }, Local { rx })
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

        let mut buf = Vec::<u8, COMMAND_BUF_SIZE>::new();

        loop {
            // Let the scheduler switch to a different task before reading (again)
            Rtc0::delay(5.millis()).await;

            if let Ok(byte) = rx.read() {
                defmt::trace!("Received '{}' via UART", char::try_from(byte));
                if byte == b';' {
                    if let Ok(cmd) = Command::try_from(buf.as_slice()) {
                        cmd.execute();
                    }
                    buf.clear();
                } else {
                    if buf.is_full() {
                        defmt::trace!("Clearing buffer due to overflow and no command detected");
                        buf.clear();
                    }

                    buf.push(byte).unwrap();
                }
            }
        }
    }

    #[task(priority = 1, shared = [ tx ])]
    async fn command_help(mut cx: command_help::Context) {
        defmt::trace!("Executing help command");

        cx.shared.tx.lock(|tx| {
            write!(
                tx,
                "\r\n\
                === command-rs ===\r\n\
                \r\n\
                This is my take at week 5's assignment of the\r\n\
                Making embedded systems course (2023 edition).\r\n\
                \r\n\
                available commands:\r\n\
                \r\n\
                help - prints this help message\r\n"
            )
            .unwrap();
            tx.flush().unwrap();
        });
    }
}
