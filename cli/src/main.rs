/*
 * Copyright 2021 Cargill Incorporated
 * Copyright 2022 Bitwise IO, Inc.
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

mod error;

use std::path::Path;

use clap::{Parser, Subcommand};
use cylinder::jwt::JsonWebTokenBuilder;
use cylinder::secp256k1::Secp256k1Context;
use cylinder::{
    current_user_key_name, current_user_search_path, load_key, load_key_from_path, Context,
    PrivateKey,
};
use flexi_logger::{LogSpecBuilder, Logger};
use log::{error, info, LevelFilter};

use error::CliError;

/// Cylinder CLI
#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
pub struct CylArgs {
    /// Increase verbosity level
    #[arg(long, short = 'v', action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Decrease verbosity level
    #[arg(long, short = 'q', alias = "silent",  action = clap::ArgAction::Count, global = true)]
    quiet: u8,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate/examine Cylinder JWT (JSON web tokens)
    Jwt {
        #[command(subcommand)]
        commands: JwtCommands,
    },
}

#[derive(Subcommand)]
enum JwtCommands {
    /// Generates a JWT for a given private key
    Generate {
        /// Name or path of private key
        #[arg(long, short = 'k')]
        key: Option<String>,
    },
}

fn main() {
    let args = CylArgs::parse();

    let log_level = {
        match args.verbose as i8 - args.quiet as i8 {
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

    let res = match args.commands {
        Commands::Jwt { commands } => match commands {
            JwtCommands::Generate { key } => handle_jwt_generate(key),
        },
    };

    if let Err(err) = res {
        error!("{}", err);
    }
}

fn handle_jwt_generate(key_name: Option<String>) -> Result<(), CliError> {
    let private_key = load_private_key(key_name)?;

    let context = Secp256k1Context::new();
    let signer = context.new_signer(private_key);

    let encoded_token = JsonWebTokenBuilder::new().build(&*signer).map_err(|err| {
        CliError::from_source_with_message(Box::new(err), "failed to build json web token".into())
    })?;

    info!("{}", encoded_token);

    Ok(())
}

fn load_private_key(key_name_opt: Option<String>) -> Result<PrivateKey, CliError> {
    if let Some(key_name) = key_name_opt.as_ref() {
        if key_name.contains('/') {
            return load_key_from_path(Path::new(&key_name))
                .map_err(|err| CliError::from_source(Box::new(err)));
        }
    }

    load_key(
        &key_name_opt
            .map(String::from)
            .unwrap_or_else(current_user_key_name),
        &current_user_search_path(),
    )
    .map_err(|err| CliError::from_source(Box::new(err)))?
    .ok_or_else(|| {
        CliError::with_message({
            format!(
                "No signing key found in {}. Specify a valid key with the --key argument",
                current_user_search_path()
                    .iter()
                    .map(|path| path.as_path().display().to_string())
                    .collect::<Vec<String>>()
                    .join(":")
            )
        })
    })
}

pub fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", record.args(),)
}
