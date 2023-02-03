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

use std::path::Path;

use cylinder::jwt::JsonWebTokenBuilder;
use cylinder::secp256k1::Secp256k1Context;
use cylinder::{
    current_user_key_name, current_user_search_path, load_key, load_key_from_path, Context,
    PrivateKey,
};

use crate::error::CliError;

pub(crate) fn handle_jwt_generate(key_name: Option<String>) -> Result<(), CliError> {
    let private_key = load_private_key(key_name)?;

    let context = Secp256k1Context::new();
    let signer = context.new_signer(private_key);

    let encoded_token = JsonWebTokenBuilder::new().build(&*signer).map_err(|err| {
        CliError::from_source_with_message(Box::new(err), "failed to build json web token".into())
    })?;

    println!("{encoded_token}");

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
