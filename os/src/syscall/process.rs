//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translate_va_to_pa,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, fetch_task_info,
        suspend_current_and_run_next, TaskStatus, TASK_MANAGER,
    },
    timer::get_time_us,
};

/// TimeVal
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// second
    pub sec: usize,
    /// microsecond
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

impl TaskInfo {
    /// Initialize a TaskInfo. Status is UnInit by default.
    pub fn new() -> Self {
        Self {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = translate_va_to_pa(current_user_token(), _ts as usize) as *mut TimeVal;
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let ti = translate_va_to_pa(current_user_token(), _ti as usize) as *mut TaskInfo;
    unsafe {
        *ti = fetch_task_info();
    }
    0
}

/// syscall memory map
// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let task = TASK_MANAGER.current_task_mut();
    let memory_set = &mut task.memory_set;
    memory_set.mmap(_start, _len, _port)
    // page_table_mmap(current_user_token(), _start, _len, _port);
    // 0
}

/// syscall memory unmap
// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let task = TASK_MANAGER.current_task_mut();
    let memory_set = &mut task.memory_set;
    memory_set.munmap(_start, _len)
    // page_table_munmap(current_user_token(), _start, _len);
    // 0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
