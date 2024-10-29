// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

#[test]
fn generate_completion_conflicts_with_subcommands() {
    utils::command::command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("encode")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the subcommand 'encode' cannot be used with '--generate-completion <SHELL>'",
        ));
    utils::command::command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("decode")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the subcommand 'decode' cannot be used with '--generate-completion <SHELL>'",
        ));
}

#[test]
fn generate_completion() {
    utils::command::command()
        .arg("--generate-completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("elvish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("nushell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("powershell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    utils::command::command()
        .arg("--generate-completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::ne(""));
}

#[test]
fn generate_completion_with_invalid_shell() {
    utils::command::command()
        .arg("--generate-completion")
        .arg("a")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--generate-completion <SHELL>'",
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
