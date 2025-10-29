# SPDX-FileCopyrightText: 2022 Shun Sakai
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

alias lint := clippy

# Run default recipe
_default:
    just -l

# Build a package
build:
    cargo build

# Remove generated artifacts
clean:
    cargo clean

# Check a package
check:
    cargo check

# Run tests
test:
    cargo test

# Run the formatter
fmt:
    cargo +nightly fmt

# Run the linter
clippy:
    cargo clippy -- -D warnings

# Apply lint suggestions
clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

# Build man pages
build-man:
    asciidoctor -b manpage docs/man/man1/*.1.adoc

# Run the linter for GitHub Actions workflow files
lint-github-actions:
    actionlint -verbose

# Run the formatter for the README
fmt-readme:
    npx prettier -w README.md

# Build the book
build-book:
    npx antora antora-playbook.yml

# Increment the version
bump part:
    bump-my-version bump {{ part }}
    cargo set-version --bump {{ part }}
