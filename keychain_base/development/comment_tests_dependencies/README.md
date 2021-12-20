# Cleaning tests

Because the rand crate doesn't compile into wasm, but it's still needed to create unique ids (tests are run on the same crate) in unit-tests, we have to comment them for deploy (to leave out the rand crate out of dependencies). This is where the development/comment_tests_dependencies tool comes in. The prepare_deploy comments the testing code, and the finish_deploy undoes all of it.

To deploy the canister with all the testing code commented out, use the deploy script.

The scripts are set up using the `settings.cfg` file

## Using settings.cfg

New files and dependencies can be added to the config to be cleaned out during deploy.

`path` -- holds the path to the directory with the lib.rs file (and where all of the source code really is). The path to the files below should start from the path that is stated here

`test_files` -- these are the test files and parts of modules that should be commented out completely

`files_with_tests` -- these are the files the lib code which happen to contain unit-tests. Here, only the test modules are to be cleaned out

`toml_files` -- this is the direction to the .toml files that happen to hold the forbidden dependencies

`dependencies` -- the list of actual forbidden dependencies
