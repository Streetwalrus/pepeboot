#![feature(alloc_system, global_allocator, allocator_api)]

extern crate alloc_system;
extern crate libc;

use alloc_system::System;

#[global_allocator]
static A: System = System;

use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<Error>> {
    println!("Hello, world!");

    unsafe {
        // TODO: mount procfs and sysfs, and don't hardcode this
        libc::mount(
            std::ffi::CString::new("dev").unwrap().as_ptr(),
            std::ffi::CString::new("/dev").unwrap().as_ptr(),
            std::ffi::CString::new("devtmpfs").unwrap().as_ptr(),
            libc::MS_NOSUID,
            std::ptr::null());
        libc::mount(
            std::ffi::CString::new("proc").unwrap().as_ptr(),
            std::ffi::CString::new("/proc").unwrap().as_ptr(),
            std::ffi::CString::new("proc").unwrap().as_ptr(),
            libc::MS_NOSUID,
            std::ptr::null());
    }

    let mut partitions = String::new();
    let mut f = std::fs::File::open("/proc/partitions")?;
    f.read_to_string(&mut partitions)?;
    print!("{}", partitions);

    for p in partitions.lines().skip(2) {
        unsafe {
            let p = format!("/dev/{}", p.split_whitespace().nth(3).unwrap());
            let rc = libc::mount(
                std::ffi::CString::new(p).unwrap().as_ptr(),
                std::ffi::CString::new("/mnt").unwrap().as_ptr(),
                std::ffi::CString::new("vfat").unwrap().as_ptr(),
                libc::MS_RDONLY,
                std::ptr::null());

            if rc == 0 {
                let l = std::fs::read_dir("/mnt")?;
                for f in l {
                    println!("{}", f?.path().display());
                }
                break
            }
        }
    }

    loop {}
}
