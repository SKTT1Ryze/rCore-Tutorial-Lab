//! interrupt handle function
use super::context::Context;
use super::handler;
use super::timer;
use crate::fs::STDIN;
use crate::kernel::syscall_handler;
use crate::process::PROCESSOR;
use crate::sbi::console_getchar;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

pub union Vector {
    pub handler: unsafe fn(context: &mut Context) -> *mut Context,
}
#[no_mangle]
pub static __INTERRUPTS: [Vector; 4] = [
    Vector {
        handler: breakpoint,
    },
    Vector {
        handler: syscall_handler,
    },
    Vector {
        handler: supervisor_timer,
    },
    Vector {
        handler: supervisor_external,
    },
];

pub fn get_handle_function(index: usize, context: &mut Context) {
    let handle_function_table = [
        |context: &mut Context| {
            println!("Breakpoint at 0x{:x}", context.sepc);
            context.sepc += 2;
        }, // breakpoint interrupt closure
        |context: &mut Context| {
            println!("system call at 0x{:x}", context.sepc);
            context.sepc += 2;
        }, //system call closure
        |context: &mut Context| {
            timer::tick();
        }, //time interrupt closure
        |context: &mut Context| {
            println!("External interrupt at 0x{:x}", context.sepc);
            context.sepc += 2;
        },
    ];
    handle_function_table[index](context);
}

/// handle ebreak interrupt
///
/// continue: sepc add 2 to continue
pub fn breakpoint(context: &mut Context) -> *mut Context {
    //println!("Breakpoint at 0x{:x}", context.sepc);
    /*
    println!("Another breakpoint interrupt start");
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    println!("Another breakpoint interrupt end");
    */
    context.sepc += 2;
    //println!("breakpoint interrupt return");
    context
}

/// handle time interrupt
///
/// count in `tick()` and call a ebreak
//pub fn supervisor_timer(_: &Context) {
pub fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.lock().park_current_thread(context);
    //println!("timer interrupt return");
    PROCESSOR.lock().prepare_next_thread()
}

/// handle external interrupt
///
/// continue: sepc add 2 to continue
pub fn supervisor_external(context: &mut Context) -> *mut Context {
    let mut c = console_getchar();
    let f = 'f' as usize;
    if c <= 255 {
        match c {
            3 => {
                PROCESSOR.lock().kill_current_thread();
                PROCESSOR.lock().prepare_next_thread();
            }
            f => {
                PROCESSOR.lock().fork_current_thread(context);
            }
            _ => {
                if c == '\r' as usize {
                    c = '\n' as usize;
                }
            }
        }
        STDIN.push(c as u8);
    }
    context
}

/// 出现未能解决的异常，终止当前线程
pub fn fault(_context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    println!(
        "{:x?} terminated with {:x?}",
        PROCESSOR.lock().current_thread(),
        scause.cause()
    );
    println!("stval: {:x}", stval);
    PROCESSOR.lock().kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    PROCESSOR.lock().prepare_next_thread()
}
