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
