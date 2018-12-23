# Const
#===============================================================
_name := cargo-launcher

# Option
#===============================================================
SHELL                   := /bin/bash
LOG_LEVEL               := debug
PREFIX                  := $(HOME)/.cargo
LOG                     := $(shell echo '$(_name)' | tr - _)=$(LOG_LEVEL)
CARGO_VERSION           := stable
CARGO_OPTIONS           :=
CARGO_SUB_OPTIONS       :=
CARGO_COMMAND           := cargo +$(CARGO_VERSION) $(CARGO_OPTIONS)
APP_ARGS                := launcher albert

# Environment
#===============================================================
export RUST_LOG=$(LOG)
export RUST_BACKTRACE=1

# Task
#===============================================================
run: ## Execute a main.rs
	$(CARGO_COMMAND) run $(CARGO_SUB_OPTIONS) $(APP_ARGS)

test: lint ## Run the tests
	$(CARGO_COMMAND) test $(CARGO_SUB_OPTIONS) -- --nocapture

check: ## Check syntax, but don't build object files
	$(CARGO_COMMAND) check $(CARGO_SUB_OPTIONS)

build: lint ## Build all project
	$(CARGO_COMMAND) build $(CARGO_SUB_OPTIONS)

update: ## Update modules
	$(CARGO_COMMAND) update

check-dep: ## Check dep version
	$(CARGO_COMMAND) outdated

clean: ## Remove the target directory
	$(CARGO_COMMAND) clean

install: ## Install to $(PREFIX) directory
	$(CARGO_COMMAND) install --force --root $(PREFIX) --path .

fmt: ## Run fmt
	$(CARGO_COMMAND) fmt

clippy: ## Run clippy
	$(CARGO_COMMAND) clippy

lint: fmt clippy ## Run fmt and clippy

release-build: lint ## Build all project
	$(MAKE) build CARGO_SUB_OPTIONS="--release"

cross-build: ## Build all platform
	-$(MAKE) build CARGO_SUB_OPTIONS="--target x86_64-apple-darwin       --release"
	-$(MAKE) build CARGO_SUB_OPTIONS="--target x86_64-pc-windows-gnu     --release"
	-$(MAKE) build CARGO_SUB_OPTIONS="--target x86_64-unknown-linux-musl --release" CROSS_COMPILE="x86_64-linux-musl-"

cross-test: ## Build all platform
	-$(MAKE) test CARGO_SUB_OPTIONS="--target x86_64-apple-darwin       --release"
	-$(MAKE) test CARGO_SUB_OPTIONS="--target x86_64-pc-windows-gnu     --release"
	-$(MAKE) test CARGO_SUB_OPTIONS="--target x86_64-unknown-linux-musl --release" CROSS_COMPILE="x86_64-linux-musl-"

bump-patch: ## Bump up patch
	$(MAKE) _bump BUMP_LEVEL=patch
bump-minor: ## Bump up minor
	$(MAKE) _bump BUMP_LEVEL=minor
bump-major: ## Bump up major
	$(MAKE) _bump BUMP_LEVEL=major

_bump:
	$(CARGO_COMMAND) bump $(BUMP_LEVEL)
	$(CARGO_COMMAND) metadata --format-version 1  > /dev/null
	git add Cargo.toml Cargo.lock
	git commit -m "Bump up version number to $$($(CARGO_COMMAND) read-manifest | jq -r '.version')"

publish: ## Publish to crates.io
	$(CARGO_COMMAND) package
	$(CARGO_COMMAND) publish

help: ## Print help
	echo -e "Usage: make [task]\n\nTasks:"
	perl -nle 'printf("    \033[33m%-20s\033[0m %s\n",$$1,$$2) if /^([a-zA-Z_-]*?):(?:.+?## )?(.*?)$$/' $(MAKEFILE_LIST)

# Config
#===============================================================
.SILENT: help
# If you want `Target` instead of `Task`, you can avoid it by using dot(.) and slash(/)
# ex) node_modules: => ./node_modules:
.PHONY: $(shell egrep -o '^(\._)?[a-z_-]+:' $(MAKEFILE_LIST) | sed 's/://')
.DEFAULT_GOAL := build
