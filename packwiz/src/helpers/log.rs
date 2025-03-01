#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LogLevel {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for u32 {
    fn from(value: LogLevel) -> Self {
        value as u32
    }
}

pub fn log(level: LogLevel, msg: String) {
    unsafe {
        let _ = crate::config::host::log(level.into(), msg);
    };
}
