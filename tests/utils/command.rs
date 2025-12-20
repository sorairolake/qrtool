// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use assert_cmd::{Command, cargo::cargo_bin_cmd};

pub fn command() -> Command {
    let mut command = cargo_bin_cmd!();
    command.current_dir("tests");
    command
}
