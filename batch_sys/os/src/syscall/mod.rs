mod process;
mod fs;

use process::*;
use fs::*;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize{
    match syscall_id {
        SYSCALL_WRITE =>{
            sys_write(args[0], args[0] as *const u8, args[0])
        }
        SYSCALL_EXIT =>{
            sys_exit(args[0] as i32)
        }
        _ =>{
            panic!("Unsupported syscall id.....")
        }
    }
}