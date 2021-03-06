# ic_chatbot

IC Chatbot is an open-source chatbot solution that bolsters native support tooling and composability for services running on the Internet Computer. Developers can implement this tool directly into their Internet Computer-based dapps and websites, providing out-of-the-box chatbot functionality as well as enhanced programmability and composability potential that conventional Web 2.0 products cannot match.

IC Chatbot can rapidly evolve from a simple support bot to provide developers and users with a much greater degree of composability, allowing for it to be freely integrated into services in different combinations for various purposes. Because of IC Chatbot’s powerful ability to interface with a growing number of different canisters on the Internet Computer, it can be expanded into a tool that spurs permissionless and trustless innovation on the network at virtually all levels of granularity.

## Running the project locally

If you want to run the project locally, you can use the following commands:

```bash
# Start the replica, running in the background.
dfx start --background

# Deploy your canisters to the replica and generate the candid interface.
dfx deploy

# Once you obtain the canister-id of ic_chatbot after deploy, open the agent/src/main.rs file,
# and in the main() function, set canister_id and replica variables accordingly. Then, change
# to the agent folder.
cd agent

# Run the agent software to populate the ic_chatbot canister's data.
cargo run

# Change back to the project root folder.
cd ..

# Install front end packages.
npm install

# Start the front end.
npm start 
```

In order to complete the steps above, you may also need to install Rust, node, etc., if not already installed.

Once the job completes, your application will be available at `http://localhost:8080`.
=======
# rust_profile

Welcome to your new rust_profile project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with rust_profile, see the following documentation available online:

- [Quick Start](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html)
- [Motoko Programming Language Guide](https://sdk.dfinity.org/docs/language-guide/motoko.html)
- [Motoko Language Quick Reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd rust_profile/
dfx help
dfx config --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

Additionally, if you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 8000.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`NODE_ENV` to `production` if you are using Webpack
- use your own preferred method to replace `process.env.NODE_ENV` in the autogenerated declarations
- Write your own `createActor` constructor# ic-chatbot
