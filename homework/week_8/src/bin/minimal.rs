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

    static mut static_mut_initialized: u8 = 42;
    static mut static_mut_uninitialized: MaybeUninit<u8> = MaybeUninit::uninit();

    macro_rules! get_dunder {
        ($dst:ident, $src:ident) => {
            let $dst: usize;
            unsafe {
                $dst = &$src as *const u8 as usize;
            }
        };
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
        get_dunder!(pre_init, __pre_init);
        get_dunder!(stack_start, _stack_start);

        get_dunder!(vector_table, __vector_table);
        get_dunder!(reset_vector, __reset_vector);
        get_dunder!(eexceptions, __eexceptions);

        get_dunder!(stext, __stext);
        get_dunder!(etext, __etext);

        get_dunder!(srodata, __srodata);
        get_dunder!(erodata, __erodata);

        get_dunder!(sdata, __sdata);
        get_dunder!(edata, __edata);

        get_dunder!(sidata, __sidata);
        get_dunder!(veneer_base, __veneer_base);
        get_dunder!(veneer_limit, __veneer_limit);

        get_dunder!(sbss, __sbss);
        get_dunder!(ebss, __ebss);

        get_dunder!(suninit, __suninit);
        get_dunder!(euninit, __euninit);

        get_dunder!(sheap, __sheap);

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

        defmt::info!("static_mut_initialized '0x{:x}'", unsafe {
            &static_mut_initialized as *const _ as usize
        });

        defmt::info!("static_mut_uninitialized '0x{:x}'", unsafe {
            &static_mut_uninitialized as *const _ as usize
        });

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
