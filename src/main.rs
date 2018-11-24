extern crate libc;
extern crate nix;

use std::alloc::System;
#[global_allocator]
static A: System = System;

use std::error::Error;
use std::io::Read;
use std::os::unix::io::AsRawFd;

use libc::{c_int,c_ulong};

unsafe fn kexec_file_load(kernel_fd: c_int, initrd_fd: c_int,
                   cmdline_len: c_ulong, cmdline: *const libc::c_char,
                   flags: c_ulong) -> libc::c_long {
    libc::syscall(libc::SYS_kexec_file_load,
                  kernel_fd, initrd_fd,
                  cmdline_len, cmdline,
                  flags) as libc::c_long
}

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

    let mut uptime = String::new();
    let mut f = std::fs::File::open("/proc/uptime")?;
    f.read_to_string(&mut uptime)?;
    print!("{}", uptime);

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
                let kernel = std::fs::File::open("/mnt/vmlinuz-linux")?;
                let initrd = std::fs::File::open("/mnt/initramfs-linux.img")?;
                let cmdline = std::ffi::CString::new("rw root=UUID=e42b82b7-d249-4b6a-9258-bac783078612 iomem=relaxed quiet").unwrap();
                let cmdline_ptr = cmdline.as_ptr();
                let cmdline_len = libc::strlen(cmdline_ptr) as u64;
                println!("{}", cmdline_len);
                let rc = kexec_file_load(kernel.as_raw_fd(), initrd.as_raw_fd(),
                                cmdline_len + 1, cmdline_ptr,
                                0);
                println!("{} {}", rc, nix::errno::errno());
                libc::reboot(libc::RB_KEXEC);
                break;
            }
        }
    }

    let mut uptime = String::new();
    let mut f = std::fs::File::open("/proc/uptime")?;
    f.read_to_string(&mut uptime)?;
    print!("{}", uptime);

    loop {}
}
