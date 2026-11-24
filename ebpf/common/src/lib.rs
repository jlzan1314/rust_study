#![no_std]

pub const EXEC: u32 = 1;
pub const EXIT: u32 = 2;

// 所有跨内核边界的类型具有固定布局、显式填充，不包含引用或 bool。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Config {
    pub uid: u32,
    pub command: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Event {
    pub timestamp_ns: u64,
    pub elapsed_ns: u64,
    pub pid: u32,
    pub uid: u32,
    pub command: [u8; 16],
    pub kind: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Stats {
    pub executions: u64,
    pub completed: u64,
    pub total_ns: u64,
    pub dropped_events: u64,
    pub tracking_errors: u64,
}

#[cfg(all(feature = "userspace", target_os = "linux"))]
// SAFETY: 固定布局、无隐式填充，且所有位模式都合法。
unsafe impl aya::Pod for Config {}
#[cfg(all(feature = "userspace", target_os = "linux"))]
unsafe impl aya::Pod for Stats {}
