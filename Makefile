# Const
#===============================================================
_name := cargo-launcher

# Option
#===============================================================
SHELL                   := /bin/bash
LOG_LEVEL               := debug
PREFIX                  := /usr/local/bin
LOG                     := $(shell echo '$(_name)' | tr - _)=$(LOG_LEVEL)
CARGO_VERSION           := stable
CARGO_OPTIONS           :=
CARGO_SUB_OPTIONS       :=
CARGO_COMMAND           := cargo +$(CARGO_VERSION) $(CARGO_OPTIONS)
APP_ARGS                := launcher hain

# Environment
#===============================================================
export RUST_LOG=$(LOG)
export RUST_BACKTRACE=1

# Target
#===============================================================
./target/release/$(_name): release-build

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

release-build: lint ## Build all project
	$(MAKE) build CARGO_SUB_OPTIONS="--release"

update: ## Update modules
	$(CARGO_COMMAND) update

check-dep: ## Check dep version
	$(CARGO_COMMAND) outdated

clean: ## Remove the target directory
	$(CARGO_COMMAND) clean

install: ./target/release/$(_name) ## Install to $(PREFIX) directory
	install -m 755 ./target/release/$(_name) $(PREFIX)

fmt: ## Run fmt
	$(CARGO_COMMAND) fmt

clippy: ## Run clippy
	$(CARGO_COMMAND) clippy

lint: fmt clippy ## Run fmt and clippy

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
