.PHONY: new run hello_world

# Create new application: `make new myapp`
new:
	cargo new $(NEW_ARGS)

# Run the example: `make run 02_hello_cargo`
# `cargo run` builds and runs the application.
# `cargo build` to just build, 
# `cargo build --release` to build the release version.
run:
	cd $(RUN_ARGS) && cargo run $(RUN_ARGS)

# This example doesn't use cargo
hello_world:
	cd 01_hello_world && rustc main.rs && ./main

# After installation, also add the following into the shell profile
# export PATH="$HOME/.cargo/bin:$PATH"
rust-install:
	curl https://sh.rustup.rs -sSf > rustup.sh
	chmod +x rustup.sh
	./rustup.sh

rust-update:
	rustup update

# Utils
#
# Handle "run" parameters, see https://stackoverflow.com/questions/2214575.
# If the first argument is "run"...
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif
#
# Handle "new" parameters, see https://stackoverflow.com/questions/2214575.
# If the first argument is "new"...
ifeq (new,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "new"
  NEW_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(NEW_ARGS):;@:)
endif
