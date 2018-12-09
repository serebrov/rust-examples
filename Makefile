# Create new application
# make new myapp
new:
	cargo new $(NEW_ARGS)

# Run the example: make run 01_hello_cargo
run:
	cd $(RUN_ARGS) && cargo run $(RUN_ARGS)

# Utils.
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
