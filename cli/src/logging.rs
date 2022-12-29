/*
 * Copyright 2021 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use flexi_logger::{LogSpecBuilder, Logger};
use log::LevelFilter;

pub(crate) fn configure_logging(log_level: i8) {
    let log_level = {
        match log_level {
            i8::MIN..=-2 => LevelFilter::Error,
            -1 => LevelFilter::Warn,
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            2..=i8::MAX => LevelFilter::Trace,
        }
    };

    let log_spec = LogSpecBuilder::new().default(log_level).build();

    match Logger::with(log_spec)
        .format(log_format)
        .log_to_stdout()
        .start()
    {
        Ok(_) => {}
        #[cfg(test)]
        // `FlexiLoggerError::Log` means the logger has already been initialized; this will happen
        // when `run` is called more than once in the tests.
        Err(flexi_logger::FlexiLoggerError::Log(_)) => {}
        Err(err) => panic!("Failed to start logger: {}", err),
    }
}

pub fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", record.args(),)
}
