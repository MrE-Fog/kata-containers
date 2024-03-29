// Copyright (c) 2022-2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use shim_interface::KATA_PATH;

// The socket used to connect to CH. This is used for CH API communications.
const CH_API_SOCKET_NAME: &str = "ch-api.sock";

// The socket that allows runtime-rs to connect direct through to the Kata
// Containers agent running inside the CH hosted VM.
const CH_VM_SOCKET_NAME: &str = "ch-vm.sock";

const CH_JAILER_DIR: &str = "root";

// Return the path for a _hypothetical_ sandbox: the path does *not* exist
// yet, and for this reason safe-path cannot be used.
pub fn get_sandbox_path(id: &str) -> Result<String> {
    let path = [KATA_PATH, id].join("/");

    Ok(path)
}

// Return the path for a _hypothetical_ API socket path:
// the path does *not* exist yet, and for this reason safe-path cannot be
// used.
pub fn get_api_socket_path(id: &str) -> Result<String> {
    let sandbox_path = get_sandbox_path(id)?;

    let path = [&sandbox_path, CH_API_SOCKET_NAME].join("/");

    Ok(path)
}

// Return the path for a _hypothetical_ sandbox specific VSOCK socket path:
// the path does *not* exist yet, and for this reason safe-path cannot be
// used.
pub fn get_vsock_path(id: &str) -> Result<String> {
    let sandbox_path = get_sandbox_path(id)?;

    let path = [&sandbox_path, CH_VM_SOCKET_NAME].join("/");

    Ok(path)
}

pub fn get_jailer_root(id: &str) -> Result<String> {
    let sandbox_path = get_sandbox_path(id)?;

    let path = [&sandbox_path, CH_JAILER_DIR].join("/");

    Ok(path)
}
