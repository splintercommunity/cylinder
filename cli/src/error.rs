// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Module containing CliError implementation.

use std::error;
use std::fmt;

struct Source {
    prefix: Option<String>,
    source: Box<dyn error::Error>,
}

/// An error which is returned for reasons internal to the function.
///
/// This error is produced when a failure occurred within the function but the failure is due to an
/// internal implementation detail of the function. This generally means that there is no specific
/// information which can be returned that would help the caller of the function recover or
/// otherwise take action.
pub struct CliError {
    message: Option<String>,
    source: Option<Source>,
}

impl CliError {
    /// Constructs a new `CliError` from a specified source error.
    ///
    /// The implementation of `std::fmt::Display` for this error will simply pass through the
    /// display of the source message unmodified.
    pub fn from_source(source: Box<dyn error::Error>) -> Self {
        Self {
            message: None,
            source: Some(Source {
                prefix: None,
                source,
            }),
        }
    }

    /// Constructs a new `CliError` from a specified source error and message string.
    ///
    /// The implementation of `std::fmt::Display` for this error will be the message string
    /// provided.
    pub fn from_source_with_message(source: Box<dyn error::Error>, message: String) -> Self {
        Self {
            message: Some(message),
            source: Some(Source {
                prefix: None,
                source,
            }),
        }
    }

    /// Constructs a new `CliError` with a specified message string.
    ///
    /// The implementation of `std::fmt::Display` for this error will be the message string
    /// provided.
    pub fn with_message(message: String) -> Self {
        Self {
            message: Some(message),
            source: None,
        }
    }
}

impl error::Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            Some(s) => Some(s.source.as_ref()),
            None => None,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(m) => write!(f, "{}", m),
            None => match &self.source {
                Some(s) => match &s.prefix {
                    Some(p) => write!(f, "{}: {}", p, s.source),
                    None => write!(f, "{}", s.source),
                },
                None => write!(f, "{}", std::any::type_name::<CliError>()),
            },
        }
    }
}

impl fmt::Debug for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Tests that errors constructed with `CliError::from_source` return a debug string exactly
    /// the same as Display.
    #[test]
    fn test_debug_from_source() {
        let msg = "test message";
        let err = CliError::from_source(Box::new(CliError::with_message(msg.to_string())));
        assert_eq!(format!("{:?}", err), format!("{}", err));
    }

    /// Tests that errors constructed with `CliError::from_source_with_message` return a debug
    /// string exactly the same as Display.
    #[test]
    fn test_debug_from_source_with_message() {
        let msg = "test message";
        let err = CliError::from_source_with_message(
            Box::new(CliError::with_message("unused".to_string())),
            msg.to_string(),
        );
        assert_eq!(format!("{:?}", err), format!("{}", err));
    }

    /// Tests that errors constructed with `CliError::with_message` return a debug
    /// string exactly the same as Display.
    #[test]
    fn test_debug_with_message() {
        let msg = "test message";
        let err = CliError::with_message(msg.to_string());
        assert_eq!(format!("{:?}", err), format!("{}", err));
    }

    /// Tests that error constructed with `CliError::from_source` return a display
    /// string which is the same as the source's display string.
    #[test]
    fn test_display_from_source() {
        let msg = "test message";
        let err = CliError::from_source(Box::new(CliError::with_message(msg.to_string())));
        assert_eq!(format!("{}", err), msg);
    }

    /// Tests that error constructed with `CliError::from_source_with_message` return
    /// message as the display string.
    #[test]
    fn test_display_from_source_with_message() {
        let msg = "test message";
        let err = CliError::from_source_with_message(
            Box::new(CliError::with_message("unused".to_string())),
            msg.to_string(),
        );
        assert_eq!(format!("{}", err), msg);
    }

    /// Tests that error constructed with `CliError::with_message` return message as the
    /// display string.
    #[test]
    fn test_display_with_message() {
        let msg = "test message";
        let err = CliError::with_message(msg.to_string());
        assert_eq!(format!("{}", err), msg);
    }
}
