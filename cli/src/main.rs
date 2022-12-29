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

mod commands;
mod error;
mod logging;

use clap::{Parser, Subcommand};

use commands::handle_jwt_generate;
use error::CliError;
use logging::configure_logging;

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

fn main() -> Result<(), CliError> {
    let args = CylArgs::parse();

    configure_logging(args.verbose as i8 - args.quiet as i8);

    match args.commands {
        Commands::Jwt { commands } => match commands {
            JwtCommands::Generate { key } => handle_jwt_generate(key),
        },
    }
}
