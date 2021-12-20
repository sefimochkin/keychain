# Keychain Base

This will become the storage part of the keychain. This is the canister with which the plugin will communicate, and where the decentralized email service will get its real addresses.

## Proposed API

Terminology:

`Keydata` -- a struct where all of the crucial data is stored. Email address, phone number(some day), etc.

`Obfuscated_data` -- a struct where the obfuscation fields for some website are stored. e.g. obfuscated email addresses, obfuscated phone numbers (some day), passwords.

`*_core` -- a struct through which the client is conversing with the backend. Basically, a `Keydata` or `Obfuscated_data` struct without all of the key fields.

`owner_id` -- an id that identifies that the user has the access to their private key. (I may be mistaken that this is the only thing needed. Metamask does this in some way, and I am currently researching it). This can be an IC::Principal, but it will make the user create an ICP's Internet Indentity first, and it can be too much. Though we can use IC::Principal as an alternative way to login.

API:

`init_keydata` -- a call to init the main data. Called when the user sets up their keychain plugin for the first time.

`init_obfuscated_data` -- a call to init an instance of obfuscated data. Called when the user chooses to mask their email/password.

`get_keydata` -- a call to get the main data. Called when logging in into a new instance of the plugin (e.g. new computer/phone)

`get_obfuscated_data` -- a call to get all of the obfuscated data. Called when logging in into a new instance of the plugin (e.g. new computer/phone)

`change_keydata` -- a call to change the main data. Called when user chooses to change their keydata (e.g. email address)


## Running the project locally (a part of the intro guide)

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

## Deploying the project locally (really)

Because the `rand` crate doesn't compile into wasm, but it's still needed to create unique ids (tests are run on the same crate) in unit-tests, we have to comment them for deploy (to leave out the `rand` crate out of dependencies).
This is where the `development/comment_tests_dependencies` tool comes in.
The `prepare_deploy` comments the testing code, and the `finish_deploy` undoes all of it.

To deploy the canister with all the testing code commented out, use the `deploy` script.

## Running commands on the canister

Below are examples of calls to the canister:

init_keydata
```bash
dfx canister call keychain_base init_keydata '(record {email_address = "email_1"; phone_number = opt "phone_1"}, "owner_1")
```

init_obfuscated_data
```bash
dfx canister call keychain_base init_obfuscated_data '(record {obfuscated_email_address = "o_email_1"; website_url = "netflix"}, "owner_1")'
```

get_keydata
```bash
dfx canister call keychain_base get_keydata '("owner_1")'
```

get_obfuscated_data
```bash
dfx canister call keychain_base get_obfuscated_data '("owner_1")'
```

change_keydata
```bash
dfx canister call keychain_base change_keydata '(record {email_address = "email_2"; phone_number = opt "phone_2"}, "owner_1")'
```

## Running unit-tests

To run unit-tests:

```bash
cargo test  --  --test-threads=1
```

Using more threads will cause races inside the ic part and consequently cause errors.

## TODO
* Add js tests to check the correctness of the interface
* Consider adding `bulk_init_obfuscated_data` -- an update call may be expensive and take a lot of time. Maybe we could init the obfuscated data at the end of the session?
* Consider using stable memory -- [Canister memory problem](https://forum.dfinity.org/t/increased-canister-smart-contract-memory/6148/37) -- for now, canisters have 4GB of instable memory and 8GB of stable. In the future, this limits upped (or lifted alltogether), but for now stable memory will be upped first with instable memory lagging begind. To counter the intermediate limits, we could create another storage architecture where stable memory used as a hard drive and instable as RAM.


## Intro Guide and Links
Welcome to your new keychain_base project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with keychain_base, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd keychain_base/
dfx help
dfx config --help
```
