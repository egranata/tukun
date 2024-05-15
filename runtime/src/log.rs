use core::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LogLevel {
    fn to_int(self) -> i32 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 100,
            LogLevel::Warning => 200,
            LogLevel::Error => 300,
            LogLevel::Fatal => 400,
        }
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

#[derive(Clone, Copy)]
pub struct LogSubsystem {
    pub name: &'static str,
    pub level: LogLevel,
}

#[macro_export]
macro_rules! log_subsystem {
    ($name:literal,$level:expr) => {
        $crate::log::LogSubsystem {
            name: $name,
            level: $level,
        }
    };
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn _print<T: fmt::Write>(writer: *mut T, args: ::core::fmt::Arguments) {
    writer
        .as_mut()
        .expect("invalid writer")
        .write_fmt(args)
        .expect("log write failed");
}

#[macro_export]
macro_rules! log_do_write {
    ($($arg:tt)*) => {
        unsafe {
            $crate::log::_print( std::ptr::addr_of_mut!(LOG_WRITER), format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($subsystem:expr, $($args:expr),*) => {
        if $crate::log::LogLevel::Debug >= $subsystem.level {
            let f = file!();
            let l = line!();
            $crate::log_do_write!("{}:{} {} {}: ", f, l, $subsystem.name, "D");
            $crate::log_do_write!("{}\n", format_args!($($args),*));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($subsystem:expr, $($args:expr),*) => {
        if $crate::log::LogLevel::Info >= $subsystem.level {
            let f = file!();
            let l = line!();
            $crate::log_do_write!("{}:{} {} {}: ", f, l, $subsystem.name, "I");
            $crate::log_do_write!("{}\n", format_args!($($args),*));
        }
    };
}

#[macro_export]
macro_rules! log_warning {
    ($subsystem:expr, $($args:expr),*) => {
        if $crate::log::LogLevel::Warning >= $subsystem.level {
            let f = file!();
            let l = line!();
            $crate::log_do_write!("{}:{} {} {}: ", f, l, $subsystem.name, "W");
            $crate::log_do_write!("{}\n", format_args!($($args),*));
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($subsystem:expr, $($args:expr),*) => {
        if $crate::log::LogLevel::Error >= $subsystem.level {
            let f = file!();
            let l = line!();
            $crate::log_do_write!("{}:{} {} {}: ", f, l, $subsystem.name, "E");
            $crate::log_do_write!("{}\n", format_args!($($args),*));
        }
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($subsystem:expr, $($args:expr),*) => {
        if $crate::log::LogLevel::Fatal >= $subsystem.level {
            let f = file!();
            let l = line!();
            $crate::log_do_write!("{}:{} {} {}: ", f, l, $subsystem.name, "F");
            $crate::log_do_write!("{}\n", format_args!($($args),*));
        }
    };
}

pub struct StderrWriter {}

impl fmt::Write for StderrWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        eprint!("{s}");
        Ok(())
    }
}
