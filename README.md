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
