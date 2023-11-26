#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::ignored_unit_patterns)]

use week_8 as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = microbit::pac,
    dispatchers = [SWI0_EGU0]
)]
mod app {
    use core::mem::MaybeUninit;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    static mut STATIC_MUT_INITIALIZED: u8 = 42;
    static mut STATIC_MUT_UNINITIALIZED: MaybeUninit<u8> = MaybeUninit::uninit();

    static STATIC_CONST_INITIALIZED: u8 = 21;
    static STATIC_CONST_UNINITIALIZED: MaybeUninit<u8> = MaybeUninit::uninit();

    // As seen at
    // https://docs.rust-embedded.org/embedonomicon/main.html#life-before-main
    // and
    // https://stackoverflow.com/questions/35882994/is-there-any-way-to-get-the-address-of-a-struct-in-rust
    macro_rules! compute_addr {
        ($src:ident) => {
            &$src as *const _ as usize
        };
    }

    macro_rules! store_addr_unsafe {
        ($dst:ident, $src:ident) => {
            $dst = compute_addr!($src);
        };
    }

    /// (address static function-local variable, address non-static function-local variable)
    unsafe fn free_function() -> (usize, usize) {
        static STATIC_LOCAL: u8 = 255;
        let local: u8 = 2;

        let static_local_addr: usize;
        let local_addr: usize;

        store_addr_unsafe!(static_local_addr, STATIC_LOCAL);
        store_addr_unsafe!(local_addr, local);

        (static_local_addr, local_addr)
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        extern "C" {
            static mut __pre_init: u8;
            static mut _stack_start: u8;

            static mut __vector_table: u8;
            static mut __reset_vector: u8;
            static mut __eexceptions: u8;

            static mut __stext: u8;
            static mut __etext: u8;

            static mut __srodata: u8;
            static mut __erodata: u8;

            static mut __sdata: u8;
            static mut __edata: u8;

            static mut __sidata: u8;
            static mut __veneer_base: u8;
            static mut __veneer_limit: u8;

            static mut __sbss: u8;
            static mut __ebss: u8;

            static mut __suninit: u8;
            static mut __euninit: u8;

            static mut __sheap: u8;
        }

        // get all the __foo values from above
        let pre_init: usize;
        let stack_start: usize;

        let vector_table: usize;
        let reset_vector: usize;
        let eexceptions: usize;

        let stext: usize;
        let etext: usize;

        let srodata: usize;
        let erodata: usize;

        let sdata: usize;
        let edata: usize;

        let sidata: usize;
        let veneer_base: usize;
        let veneer_limit: usize;

        let sbss: usize;
        let ebss: usize;

        let suninit: usize;
        let euninit: usize;

        let sheap: usize;

        let global_static_mut_init: usize;
        let global_static_mut_uninit: usize;

        let global_static_const_init: usize;
        let global_static_const_uninit: usize;

        let function_static_local: usize;
        let function_local: usize;

        unsafe {
            store_addr_unsafe!(pre_init, __pre_init);
            store_addr_unsafe!(stack_start, _stack_start);

            store_addr_unsafe!(vector_table, __vector_table);
            store_addr_unsafe!(reset_vector, __reset_vector);
            store_addr_unsafe!(eexceptions, __eexceptions);

            store_addr_unsafe!(stext, __stext);
            store_addr_unsafe!(etext, __etext);

            store_addr_unsafe!(srodata, __srodata);
            store_addr_unsafe!(erodata, __erodata);

            store_addr_unsafe!(sdata, __sdata);
            store_addr_unsafe!(edata, __edata);

            store_addr_unsafe!(sidata, __sidata);
            store_addr_unsafe!(veneer_base, __veneer_base);
            store_addr_unsafe!(veneer_limit, __veneer_limit);

            store_addr_unsafe!(sbss, __sbss);
            store_addr_unsafe!(ebss, __ebss);

            store_addr_unsafe!(suninit, __suninit);
            store_addr_unsafe!(euninit, __euninit);

            store_addr_unsafe!(sheap, __sheap);

            store_addr_unsafe!(global_static_mut_init, STATIC_MUT_INITIALIZED);
            store_addr_unsafe!(global_static_mut_uninit, STATIC_MUT_UNINITIALIZED);

            store_addr_unsafe!(global_static_const_init, STATIC_CONST_INITIALIZED);
            store_addr_unsafe!(global_static_const_uninit, STATIC_CONST_UNINITIALIZED);

            (function_static_local, function_local) = free_function();
        }

        // dump all the __foo values via RTT
        defmt::info!("pre_init '0x{:x}'", pre_init);
        defmt::info!("stack_start '0x{:x}'", stack_start);

        defmt::info!("vector_table '0x{:x}'", vector_table);
        defmt::info!("reset_vector '0x{:x}'", reset_vector);
        defmt::info!("eexceptions '0x{:x}'", eexceptions);

        defmt::info!("stext '0x{:x}'", stext);
        defmt::info!("etext '0x{:x}'", etext);

        defmt::info!("srodata '0x{:x}'", srodata);
        defmt::info!("erodata '0x{:x}'", erodata);

        defmt::info!("sdata '0x{:x}'", sdata);
        defmt::info!("edata '0x{:x}'", edata);

        defmt::info!("sidata '0x{:x}'", sidata);
        defmt::info!("veneer_base '0x{:x}'", veneer_base);
        defmt::info!("veneer_limit '0x{:x}'", veneer_limit);

        defmt::info!("sbss '0x{:x}'", sbss);
        defmt::info!("ebss '0x{:x}'", ebss);

        defmt::info!("suninit '0x{:x}'", suninit);
        defmt::info!("euninit '0x{:x}'", euninit);

        defmt::info!("sheap '0x{:x}'", sheap);

        defmt::info!(
            "global static mut initialized '0x{:x}'",
            global_static_mut_init
        );
        defmt::info!(
            "global static mut uninitialized '0x{:x}'",
            global_static_mut_uninit
        );

        defmt::info!(
            "global static const initialized '0x{:x}'",
            global_static_const_init
        );
        defmt::info!(
            "global static const uninitialized '0x{:x}'",
            global_static_const_uninit
        );

        defmt::info!("function local static '0x{:x}'", function_static_local);
        defmt::info!("function local '0x{:x}'", function_local);

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        #[allow(clippy::empty_loop)]
        loop {}
    }
}
