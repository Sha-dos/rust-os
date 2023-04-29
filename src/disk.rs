use core::arch::asm;
use core::ptr::{write, write_bytes, read};
use core::mem::size_of;

pub const BLOCK_SIZE: usize = 4096;
pub const FILE_PATH: &str = "example.txt";

#[repr(C)]
pub struct Disk {
    fd: i32,
}

impl Disk {
    pub(crate) fn open(file_path: &str) -> Self {
        let fd = unsafe { open(file_path) };
        Disk { fd }
    }

    pub(crate) fn read(&mut self, block_num: usize, buffer: &mut [u8]) -> Result<usize, i32> {
        let bytes_read = unsafe { read_out(self.fd, buffer.as_mut_ptr(), buffer.len(), (block_num * BLOCK_SIZE) as i64) };
        if bytes_read < 0 {
            Err(-bytes_read)
        } else {
            Ok(bytes_read as usize)
        }
    }

    pub(crate) fn write(&mut self, block_num: usize, buffer: &[u8]) -> Result<usize, i32> {
        let bytes_written = unsafe { write_out(self.fd, buffer.as_ptr(), buffer.len(), (block_num * BLOCK_SIZE) as i64) };
        if bytes_written < 0 {
            Err(-bytes_written)
        } else {
            Ok(bytes_written as usize)
        }
    }
}

#[allow(non_snake_case)]
unsafe fn open(file_path: &str) -> i32 {
    const O_RDWR: i32 = 0x02;
    const O_CREAT: i32 = 0x40;
    const O_TRUNC: i32 = 0x200;

    let file_path_bytes = file_path.as_bytes();
    let mut mode = 0o666;
    let mut flags = O_RDWR | O_CREAT | O_TRUNC;

    let fd = syscall(2, file_path_bytes.as_ptr() as i32, flags as *const i8, mode, 0) as i32;
    fd
}

#[allow(non_snake_case)]
unsafe fn read_out(fd: i32, buf: *mut u8, count: usize, offset: i64) -> i32 {
    let bytes_read = syscall(0, fd, buf as *mut i8, count, offset);
    bytes_read
}

#[allow(non_snake_case)]
unsafe fn write_out(fd: i32, buf: *const u8, count: usize, offset: i64) -> i32 {
    let bytes_written = syscall(1, fd, buf as *const i8, count, offset);
    bytes_written
}

#[allow(non_snake_case)]
unsafe fn syscall(num: i32, arg1: i32, arg2: *const i8, arg3: usize, arg4: i64) -> i32 {
    let mut result: i32;
    asm!(
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    in("r10") arg4,
    lateout("rax") result,
    );
    result
}