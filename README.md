# NFTAA Proposal

This repository contains relevant resources, technical components, and documentation related to the Polkadot NFTAA proposal [submitted to the Web3 Foundation](https://github.com/w3f/Grants-Program/blob/master/applications/nftaa.md).
The proposal aims to introduce the NFTAA pallet, enhancing the Polkadot ecosystem's NFT capabilities.
For more information, as well as details on the required runtime configuration, please refer to the [README in the pallet repository](https://github.com/decenzio/pallet-nftaa).

## How to run?

### Polkadot-sdk Node

#### Docker

To run our Polkadot-SDK, follow these steps:

1. **Clone the repository**:
   ```sh
   git clone https://github.com/decenzio/polkadot-nftaa-proposal.git --recurse-submodules
   ```

2. **Navigate to the project directory**:
   ```bash
   cd polkadot-nftaa-proposal
   ```

3. **You need to build image first using**:
   ```sh
   docker build --platform linux/amd64 -t polkadot-sdk-image:latest .
   ```
   or (use no cache)
   ```sh
   docker build --platform linux/amd64 --no-cache -t polkadot-sdk-image:latest .
   ```

4. **And than run zombienet instance with**:
   ```sh
   docker run --platform linux/amd64 -p 9910-9913:9910-9913 -p 9920-9921:9920-9921 -p 9615-9620:9615-9620 --rm -it polkadot-sdk-image:latest
   ```


   <details>
      <summary>If you see an error like “Error fetching metrics from: http://127.0.0.1:9620/metrics” wait a few seconds. The whole network should start up, and you should see something like this in the console (open for image)</summary>
      <img width="943" height="1079" alt="image" src="https://github.com/user-attachments/assets/7827d006-a90d-4fb8-ac25-204d719a36bb" />

   </details>

#### Local run (withou docker)
<details>
   <summary>Local tutorial</summary>
   Alternatively for local development we recommend download our fork of polkadot-sdk
   
      ```sh
      git clone https://github.com/decenzio/polkadot-sdk.git --branch dev --recurse-submodules
      ```
   
   Inside folder `binaries` download [zombienet](https://github.com/paritytech/zombienet/releases) instance. Then add execution rights:
      ```sh
      cd binaries
      chmod +x ./zombienet
      ```
   
   Then you need to build relay node:
      ```sh
      cargo b -r -p polkadot
      ```
   
   And parachain template:
      ```sh
      cargo build --release -p parachain-template-node
      ```
   
   After that we can launch nodes with zombienet:
      ```sh
      cd binaries
      ./zombienet -p native -c 1 spawn config.toml
      ```
</details>

For now, you can interact with the pallet using the following link: [Polkadot.js Explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9920#/explorer).

### Indexer and FE

When Polkadot-sdk Node is running we can run Indexer and then FE.

#### Indexer

To run the indexer, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/decenzio/stick.git
   ```
2. **Navigate to the project directory**:
   ```bash
   cd stick
   ```
3. **Start the services using Docker**:
   ```bash
   docker compose up
   ```

Once the Docker containers are running, your indexer should be up and operational. For more information, visit our [Stick repository](https://github.com/decenzio/stick).

#### Kodadot Frontend

To run the frontend, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone --branch feat/nftaa-support https://github.com/decenzio/nft-gallery.git
   ```

2. **Navigate to the project directory**:
   ```bash
   cd nft-gallery
   ```

3. **Copy the environment file**:
   ```bash
   cp .env.example .env
   ```
   Now you can edit the .env file with your own configuration values.
   
4. **Run docker**:
   ```bash
   docker compose up
    ```

<details>
  <summary>Or run local</summary>

  Install dependencies:

  ```bash
  pnpm install
  ```
  Start the development server:
  ```bash
  pnpm dev
  ```
</details>

Once the server is running, your FE should be up and operational (http://localhost:9090/). For more information, visit our [Kodadot repository](https://github.com/decenzio/nft-gallery).

## Testing

### Manual
Manual testing can currently be performed by directly calling pallet methods following the procedure outlined in the [Indexer and FE](#indexer-and-fe).

Here is an example of the basic flow:
1. Call `create` to create an NFTAA collection (similar to creating a standard NFT collection).
2. Mint an NFTAA using the `mint` function (similar to minting a standard NFT).
3. Find the address of the created NFTAA in the event list.
4. Using the same account as the NFTAA owner, use `proxy_call` to perform actions via the new NFTAA. For example, you can call `system remark`.
5. Change to a random account (one that is not the NFTAA owner) and try step 4 again. You should encounter an error.
6. Transfer the NFTAA via `transfer` (similar to transferring a standard NFT) to a new account. Then try step 4 from the old account; you should encounter an error.
7. Switch to the new owner of the NFTAA and retry step 4. This should work without any errors.

> Testing video showcase [here](https://youtu.be/WVCoaLYoxes)

### Run Automatic Tests

<img width="684" alt="image" src="testing/nftaa-pallet-test-cargo.png"/>


To run automatic tests, follow these steps:

1. Check your Rust version:
   ```sh
   rustc --version           
   rustc 1.84.0 (9fc6b4312 2025-01-07)
   ```

2. Clone our `polkadot-sdk` fork along with its submodules:
   ```sh
   git clone https://github.com/decenzio/polkadot-sdk.git --branch dev --recurse-submodules
   ```

3. Navigate to the `polkadot-sdk` folder and run:
   ```sh
   cargo test -p pallet-nftaa
   ```

In case of problems, check the Rust version (point 0.) and use the following commands to update if necessary:

   ```sh
   rustup default stable
   rustup update
   rustup target add wasm32-unknown-unknown
   ```

   ```sh
   rustup update nightly
   rustup target add wasm32-unknown-unknown --toolchain nightly
   ```
