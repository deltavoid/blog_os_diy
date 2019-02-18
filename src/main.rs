#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use blog_os_diy::println;
use blog_os_diy::print;


/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
//#[cfg(not(test))]
//#[no_mangle] // don't mangle the name of this function
//pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//    println!("Hello World{}", "!");
//
//    blog_os_diy::gdt::init_gdt();
//    blog_os_diy::interrupts::init_idt();
//    unsafe {
//        blog_os_diy::interrupts::PICS.lock().initialize()
//    };
//    x86_64::instructions::interrupts::enable();
//
//    //blog_os_diy::trigger_a_page_fault();
//
//    use blog_os_diy::memory::{self, translate_addr};
//
//    use blog_os_diy::memory::{create_example_mapping, EmptyFrameAllocator};
//
//    const LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
//    let mut recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };
//
//    create_example_mapping(&mut recursive_page_table, &mut EmptyFrameAllocator);
//    unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e)};
//
//    println!("It did not crash!");
//    blog_os_diy::hlt_loop();
//}

use bootloader::{bootinfo::BootInfo, entry_point};

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    blog_os_diy::gdt::init_gdt();
    blog_os_diy::interrupts::init_idt();
    unsafe { blog_os_diy::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();


    use blog_os_diy::memory::{self, translate_addr, create_example_mapping};
    use x86_64::structures::paging::{PageTable, RecursivePageTable};

    let mut recursive_page_table = unsafe {
        memory::init(boot_info.p4_table_addr as usize)
    };
    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    blog_os_diy::memory::create_example_mapping(&mut recursive_page_table, &mut frame_allocator);
    unsafe { (0xdeadbeaf900 as *mut u64).write_volatile(0xf021f077f065f04e)};
    // unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e)};

    println!("It did not crash!");
    blog_os_diy::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os_diy::hlt_loop();
}