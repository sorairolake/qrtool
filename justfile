# SPDX-FileCopyrightText: 2022 Shun Sakai
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

alias all := default
alias lint := clippy

# Run default recipe
default: build

# Build a package
@build:
    cargo build

# Remove generated artifacts
@clean:
    cargo clean

# Check a package
@check:
    cargo check

# Run tests
@test:
    cargo test

# Run the formatter
@fmt:
    cargo fmt

# Run the formatter with options
@fmt-with-options:
    cargo +nightly fmt

# Run the linter
@clippy:
    cargo clippy -- -D warnings

# Apply lint suggestions
@clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

# Run the linter for GitHub Actions workflow files
@lint-github-actions:
    actionlint -verbose

# Run the formatter for the README
@fmt-readme:
    npx prettier -w README.md

# Build the book
@build-book:
    npx antora antora-playbook.yml

# Increment the version
@bump part:
    bump-my-version bump {{ part }}
    cargo set-version --bump {{ part }}
