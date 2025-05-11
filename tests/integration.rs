// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn without_subcommand() {
    utils::command::command()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "requires a subcommand but one was not provided",
        ));
}

#[test]
fn long_version() {
    utils::command::command()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help() {
    utils::command::command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/after-long-help.md"
        )));
}
