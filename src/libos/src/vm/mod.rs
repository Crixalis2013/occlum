use super::*;
use fs::{File, FileDesc, FileRef};
use process::{Process, ProcessRef};
use std::fmt;

mod process_vm;
mod user_space_vm;
mod vm_layout;
mod vm_manager;
mod vm_range;

use self::vm_layout::VMLayout;
use self::vm_manager::{VMManager, VMMapOptionsBuilder};

pub use self::process_vm::{MMapFlags, ProcessVM, ProcessVMBuilder, VMPerms};
pub use self::vm_range::VMRange;

pub fn do_mmap(
    addr: usize,
    size: usize,
    perms: VMPerms,
    flags: MMapFlags,
    fd: FileDesc,
    offset: usize,
) -> Result<usize> {
    if flags.contains(MMapFlags::MAP_ANONYMOUS) {
        debug!(
            "mmap: addr: {:#x}, size: {:#x}, perms: {:?}, flags: {:?}",
            addr, size, perms, flags,
        );
    } else {
        debug!(
            "mmap: addr: {:#x}, size: {:#x}, perms: {:?}, flags: {:?}, fd: {:?}, offset: {:?}",
            addr, size, perms, flags, fd, offset
        );
    }

    let current = current!();
    let mut current_vm = current.vm().lock().unwrap();
    current_vm.mmap(addr, size, perms, flags, fd, offset)
}

pub fn do_munmap(addr: usize, size: usize) -> Result<()> {
    debug!("munmap: addr: {:#x}, size: {:#x}", addr, size);
    let current = current!();
    let mut current_vm = current.vm().lock().unwrap();
    current_vm.munmap(addr, size)
}

pub fn do_brk(addr: usize) -> Result<usize> {
    debug!("brk: addr: {:#x}", addr);
    let current = current!();
    let mut current_vm = current.vm().lock().unwrap();
    current_vm.brk(addr)
}

pub const PAGE_SIZE: usize = 4096;
