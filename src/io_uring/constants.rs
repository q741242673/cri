#![allow(unused)]
pub const IORING_OP_NOP: u32 = 0;
pub const IORING_OP_READV: u32 = 1;
pub const IORING_OP_WRITEV: u32 = 2;
pub const IORING_OP_FSYNC: u32 = 3;
pub const IORING_OP_READ_FIXED: u32 = 4;
pub const IORING_OP_WRITE_FIXED: u32 = 5;
pub const IORING_OP_POLL_ADD: u32 = 6;
pub const IORING_OP_POLL_REMOVE: u32 = 7;
pub const IORING_OP_SYNC_FILE_RANGE: u32 = 8;
pub const IORING_OP_SENDMSG: u32 = 9;
pub const IORING_OP_RECVMSG: u32 = 10;
pub const IORING_OP_TIMEOUT: u32 = 11;
pub const IORING_OP_TIMEOUT_REMOVE: u32 = 12;
pub const IORING_OP_ACCEPT: u32 = 13;
pub const IORING_OP_ASYNC_CANCEL: u32 = 14;
pub const IORING_OP_LINK_TIMEOUT: u32 = 15;
pub const IORING_OP_CONNECT: u32 = 16;
pub const IORING_OP_FALLOCATE: u32 = 17;
pub const IORING_OP_OPENAT: u32 = 18;
pub const IORING_OP_CLOSE: u32 = 19;
pub const IORING_OP_FILES_UPDATE: u32 = 20;
pub const IORING_OP_STATX: u32 = 21;
pub const IORING_OP_LAST: u32 = 22;
pub const IOSQE_FIXED_FILE: u32 = 1;
pub const IOSQE_IO_DRAIN: u32 = 2;
pub const IOSQE_IO_LINK: u32 = 4;
pub const IOSQE_IO_HARDLINK: u32 = 8;
pub const IOSQE_ASYNC: u32 = 16;
pub const IORING_SETUP_IOPOLL: u32 = 1;
pub const IORING_SETUP_SQPOLL: u32 = 2;
pub const IORING_SETUP_SQ_AFF: u32 = 4;
pub const IORING_SETUP_CQSIZE: u32 = 8;
pub const IORING_FSYNC_DATASYNC: u32 = 1;
pub const IORING_TIMEOUT_ABS: u32 = 1;
pub const IORING_OFF_SQ_RING: u32 = 0;
pub const IORING_OFF_CQ_RING: u32 = 134217728;
pub const IORING_OFF_SQES: u32 = 268435456;
pub const IORING_SQ_NEED_WAKEUP: u32 = 1;
pub const IORING_ENTER_GETEVENTS: u32 = 1;
pub const IORING_ENTER_SQ_WAKEUP: u32 = 2;
pub const IORING_FEAT_SINGLE_MMAP: u32 = 1;
pub const IORING_FEAT_NODROP: u32 = 2;
pub const IORING_FEAT_SUBMIT_STABLE: u32 = 4;
pub const IORING_REGISTER_BUFFERS: u32 = 0;
pub const IORING_UNREGISTER_BUFFERS: u32 = 1;
pub const IORING_REGISTER_FILES: u32 = 2;
pub const IORING_UNREGISTER_FILES: u32 = 3;
pub const IORING_REGISTER_EVENTFD: u32 = 4;
pub const IORING_UNREGISTER_EVENTFD: u32 = 5;
pub const IORING_REGISTER_FILES_UPDATE: u32 = 6;