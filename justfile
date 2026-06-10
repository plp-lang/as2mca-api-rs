#-----------------------------------------------------------------------------------------------------------------------
# Justfile
#
# https://just.systems/man/en/
# https://github.com/casey/just
#-----------------------------------------------------------------------------------------------------------------------

prepare:
  cargo install --locked cargo-edit watchexec-cli

update:
  cargo update

upgrade: update
  cargo upgrade --incompatible
  just build

fmt:
  cargo fmt --all --check

fmt-fix:
  cargo fmt --all

check:
  cargo check --all-targets

lint: check
  cargo clippy --all-targets --all-features -- -D warnings

lint-fix: check
  cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

test:
  cargo test

build: lint-fix fmt-fix test
  cargo build

clean:
  cargo clean
