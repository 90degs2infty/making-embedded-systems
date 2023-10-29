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
            gpio::{
                p0::{P0_15, P0_19, P0_21, P0_22, P0_24},
                Output, PushPull,
            },
            prelude::{OutputPin, StatefulOutputPin},
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
        ToggleDisplay,
        LogBoard,
        Version,
    }

    impl Command {
        fn execute(self) {
            match self {
                Command::Help => command_help::spawn().unwrap(),
                Command::ToggleDisplay => command_toggle_display::spawn().unwrap(),
                Command::LogBoard => command_log_board::spawn().unwrap(),
                Command::Version => command_version::spawn().unwrap(),
            }
        }
    }

    impl TryFrom<&[u8]> for Command {
        type Error = &'static str;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            match value {
                b"help" => Ok(Command::Help),
                b"toggle" => Ok(Command::ToggleDisplay),
                b"log_board" => Ok(Command::LogBoard),
                b"version" => Ok(Command::Version),
                _ => Err("not a command"),
            }
        }
    }

    struct DisplayRows {
        pub row1: P0_21<Output<PushPull>>,
        pub row2: P0_22<Output<PushPull>>,
        pub row3: P0_15<Output<PushPull>>,
        pub row4: P0_24<Output<PushPull>>,
        pub row5: P0_19<Output<PushPull>>,
    }

    #[shared]
    struct Shared {
        // Better make this local to one sender task which gets its workload from a queue
        tx: UarteTx<UARTE0>,
    }

    #[local]
    struct Local {
        rx: UarteRx<UARTE0>,
        rows: DisplayRows,
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

        let mut display = board.display_pins;
        display.col1.set_low().unwrap();
        display.col2.set_low().unwrap();
        display.col3.set_low().unwrap();
        display.col4.set_low().unwrap();
        display.col5.set_low().unwrap();

        let rows = DisplayRows {
            row1: display.row1,
            row2: display.row2,
            row3: display.row3,
            row4: display.row4,
            row5: display.row5,
        };

        command_client::spawn().ok();

        (Shared { tx }, Local { rx, rows })
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
                help - prints this help message\r\n\
                toggle - toggles the entire 5x5 LED matrix\r\n\
                log_board - logs the entire game board to serial\r\n\
                version - prints VCS information\r\n\
                \r\n\
                ==================\r\n"
            )
            .unwrap();

            nb::block!(tx.flush()).unwrap();
        });
    }

    macro_rules! toggle {
        ( $pin:expr ) => {
            if $pin.is_set_high().unwrap() {
                $pin.set_low().unwrap()
            } else {
                $pin.set_high().unwrap()
            }
        };
    }

    #[task(priority = 1, local = [ rows ])]
    async fn command_toggle_display(cx: command_toggle_display::Context) {
        defmt::trace!("Executing toggle display command");

        let rows = cx.local.rows;
        toggle!(rows.row1);
        toggle!(rows.row2);
        toggle!(rows.row3);
        toggle!(rows.row4);
        toggle!(rows.row5);
    }

    #[task(priority = 1, shared = [ tx ])]
    async fn command_log_board(mut cx: command_log_board::Context) {
        defmt::trace!("Executing log board command");

        cx.shared.tx.lock(|tx| {
            write!(
                tx,
                "\r\n\
                === board ===\r\n\
                \r\n\
                As of now, the engine has not been implemented\r\n\
                so there is no board to log - I'm sorry :/\r\n\
                \r\n\
                =============\r\n"
            )
            .unwrap();

            nb::block!(tx.flush()).unwrap();
        });
    }

    #[task(priority = 1, shared = [ tx ])]
    async fn command_version(mut cx: command_version::Context) {
        defmt::trace!("Executing version command");

        cx.shared.tx.lock(|tx| {
            write!(
                tx,
                "\r\n\
                This command-rs build is based on '{}'.\r\n",
                env!("VERGEN_GIT_DESCRIBE")
            )
            .unwrap();

            nb::block!(tx.flush()).unwrap();
        });
    }
}
