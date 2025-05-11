// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn completion() {
    utils::command::command()
        .arg("completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("completion")
        .arg("elvish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("completion")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("completion")
        .arg("nushell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("completion")
        .arg("powershell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::ne(""));
}

#[test]
fn completion_with_invalid_shell() {
    utils::command::command()
        .arg("completion")
        .arg("a")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("invalid value 'a' for '<SHELL>'"));
}

#[test]
fn long_version_for_completion_command() {
    utils::command::command()
        .arg("completion")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_completion_command() {
    utils::command::command()
        .arg("completion")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/completion-after-long-help.md"
        )));
}
