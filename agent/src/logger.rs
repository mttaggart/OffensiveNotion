use litcrypt::lc;

pub const LOG_DEBUG: u64 = 5;
pub const LOG_INFO: u64  = 4;
pub const LOG_WARN: u64  = 3;
pub const LOG_ERR: u64   = 2;
pub const LOG_CRIT: u64  = 1;
pub const LOG_NONE: u64  = 0;

type LogLevel = u64;

/// Helper struct to manage logging across the tool.
/// 
/// Logging has 4 levels: `LOG_INFO`, `LOG_WARN`, `LOG_ERR`, and `LOG_CRIT`,
/// Defined in the named consts. 
pub struct Logger {
    log_level: LogLevel
}

impl Logger {

    /// Creates a new `Logger` instance.
    pub fn new(level: LogLevel) -> Logger {

        Logger {log_level: level}
    }

    /// The actual function that logs to stdout
    pub fn log(&self, log_level: LogLevel, msg: String) {
        let mut log_msg: String = match log_level {
            LOG_DEBUG => lc!("[?] "),
            LOG_INFO  => lc!("[+] "),
            LOG_WARN  => lc!("[-] "),
            LOG_ERR   => lc!("[*] "),
            LOG_CRIT  => lc!("[!] "),
            _        => "".to_string(),
        };
        log_msg.push_str(msg.as_str());
        println!("{log_msg}");
        
    }

    pub fn debug(&self, msg: String) {
        if self.log_level >= LOG_DEBUG {
            self.log(LOG_DEBUG, msg);
        }
    }

    pub fn info(&self, msg: String) {
        if self.log_level >= LOG_INFO {
            self.log(LOG_INFO, msg);
        }
    }
    
    pub fn warn(&self, msg: String) {
        if self.log_level >= LOG_WARN {
            self.log(LOG_WARN, msg);
        }
    }
    
    pub fn err(&self, msg: String) {
        if self.log_level >= LOG_ERR {
            self.log(LOG_ERR, msg);
        }
    }

    pub fn crit(&self, msg: String) {
        if self.log_level >= LOG_CRIT {
            self.log(LOG_CRIT, msg);
        }
    } 

}

macro_rules! log_out {
    ($s:tt) => {
        lc!($s)
    };
    ($s:tt, $($e:expr),*) => {{
        let mut res = lc!($s);
        $(
            res.push(' ');
            res.push_str($e);
        )*
        res
    }}
    
}
pub(crate) use log_out;