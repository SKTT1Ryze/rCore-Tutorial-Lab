#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::console::*;

#[no_mangle]
pub fn main() {
    println!("---------\\        ----------      ------        -----------");
    println!("|         \\     --              --      --      --");
    println!("|         \\   --              --          --    --");
    println!("|--------\\   --               --          --    -----------");
    println!("|\\            --              --          --    --");
    println!("|  \\            --              --      --      --");
    println!("|    \\            ----------      ------        -----------");
    println!("hustccc@rCore");
    println!("OS: Rust OS");
    println!("Kernel: rCore-Tutorial 1.0");
    println!("Shell: None");
    println!("Bootloader: OpenSBI");
    println!("CPU: **");
    println!("GPU: **");
}