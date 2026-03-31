cfg_if::cfg_if! {
    if #[cfg(any(feature = "log-rtt"))] {
        use log::{Level, Metadata, Record, LevelFilter};
        use rtt_target::{rprintln, rtt_init_print};
        use panic_probe as _;

        struct Logger {
            level: Level,
        }

        static LOGGER: Logger = Logger {
            level: Level::Info,
        };

        pub(crate) fn init() {
            rtt_init_print!();
            log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).unwrap();
        }

        impl log::Log for Logger {
            fn enabled(&self, metadata: &Metadata) -> bool {
                metadata.level() <= self.level

            }

            fn log(&self, record: &Record) {
                rprintln!("{} - {}", record.level(), record.args());
            }

            fn flush(&self) {}
        }
    }
    else {
        use panic_probe as _;
        pub(crate) fn init() {}
    }
}
