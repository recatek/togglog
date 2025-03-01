#[cfg(feature = "enabled")]
mod logging {
    pub use log::debug;
    pub use log::error;
    pub use log::info;
    pub use log::trace;
    pub use log::warn;

    pub use log::log;
    pub use log::log_enabled;
}

#[cfg(not(feature = "enabled"))]
mod logging {
    /// See [`log::debug`](https://docs.rs/log/latest/log/macro.debug.html).
    #[macro_export]
    macro_rules! debug {
        (target: $target:expr, $($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
        ($($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
    }

    /// See [`log::error!`](https://docs.rs/log/latest/log/macro.error.html).
    #[macro_export]
    macro_rules! error {
        (target: $target:expr, $($args:expr),*  $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
        ($($args:expr),*  $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
    }

    /// See [`log::info`](https://docs.rs/log/latest/log/macro.info.html).
    #[macro_export]
    macro_rules! info {
        (target: $target:expr, $($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
        ($($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
    }

    /// See [`log::trace`](https://docs.rs/log/latest/log/macro.trace.html).
    #[macro_export]
    macro_rules! trace {
        (target: $target:expr, $($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
        ($($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
    }

    /// See [`log::warn!`](https://docs.rs/log/latest/log/macro.warn.html).
    #[macro_export]
    macro_rules! warn {
        (target: $target:expr, $($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
        ($($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
    }

    /// See [`log::log`](https://docs.rs/log/latest/log/macro.log.html).
    #[macro_export]
    macro_rules! log {
        (target: $target:expr, $lvl:expr, $($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
        ($lvl:expr, $($args:expr),* $(,)?) => {
            {
                $(_ = ($args);)*
            }
        };
    }

    /// See [`log::log_enabled`](https://docs.rs/log/latest/log/macro.log_enabled.html)
    #[macro_export]
    macro_rules! log_enabled {
        (target: $target:expr, $lvl:expr $(,)?) => {
            false
        };
        ($lvl:expr $(,)?) => {
            false
        };
    }
}

pub use logging::*;
