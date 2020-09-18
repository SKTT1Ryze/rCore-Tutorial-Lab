# rCore-Tutorial Lab

Writing an OS with [Rust](https://www.rust-lang.org/) on [RISC-V](https://riscv.org/).  

## What is rCore-Tutorial

rCore-Tutorial is a OS lab designed by Tsinghua University. Get more details from this page: [rCore-Tutorial](https://github.com/rcore-os/rCore-Tutorial).  
rCore is a interesting and fully functional OS writing with Rust language on RISC-V architecture by Tsinghua students. Learn more from this page: [rCore](https://github.com/rcore-os/rCore).  
It' s pretty cool to writing an OS starting from scratch.  

## Files Description

+ Lab0～Lab6：code from lab1 ~ lab6
+ clean.sh：shall script to run ` cargo clean ` for all crates
+ path：file for supporting ` clean.sh `
+ report.md：report for the lab1 ~ lab6
+ README.md：just this file

## What I have done

+ All code from lab1 ~ lab6
+ [Segment Tree Allocator](./Lab6/os/src/algorithm/src/allocator/segment_tree_allocator.rs)
+ Implement ` VectorAllocator `trait with buddy system：[Buddy System Allocator](./Lab6/os/src/algorithm/src/allocator/buddy_system_vector_allocator.rs)
+ Ctrl + C to kill current process：[Kill Current Process](./Lab6/os/src/interrupt/handle_function.rs)
+ ` fork() `for ：[Fork Current Process](./Lab6/os/src/process/thread.rs)
+ Implement` Stride Scheduling `cheduling algorithm：[Stride Scheduler](./Lab6/os/src/algorithm/src/scheduler/stride_scheduler.rs)
+ ` sys_get_id `syscall：[sys_get_id](./Lab6/os/src/kernel/process.rs)
+ ` sys_fork `syscall：[sys_fork](./Lab6/os/src/kernel/process.rs)
+ ` sys_open `syscall：[sys_open](./Lab6/os/src/kernel/fs.rs)
+ Implement` free list `alloc algorithm：[free_list](./Lab6/os/src/algorithm/src/allocator/free_list_allocator.rs)
+ Implement` IDT `：[IDT](./Lab6/os/src/interrupt/idt.rs)  

More implement details in：[rCore Lab Report](./report.md)  

## Run rCore-Tutorial on k210 
I tried to run this rCore-Tutorial on [Kendryte k210](https://www.seeedstudio.com/blog/2019/09/12/get-started-with-k210-hardware-and-programming-environment/), and succeed.  
![k210](./k210.png)  
Thanks to @wyfcyx 's work.  
