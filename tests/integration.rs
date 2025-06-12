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
