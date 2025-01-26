# NFTAA Proposal

This is proposal for the NFTAA pallet. For more informations as same as required runtime configuration check [readme in pallet repo](https://github.com/decenzio/nftaa/blob/main/README.md).

## How to run?

### Docker
For dockerized version you need to build image first using:

```sh
docker build --platform linux/x86_64 -t polkadot-sdk-image:latest .
```

And than run zombienet instance with:

```sh
docker run --platform linux/x86_64 -p 9910-9913:9910-9913 -p 9920-9921:9920-9921 --rm -it polkadot-sdk-image:latest
```

### Local development

Alternatively for local development we recommend download fork of polkadot-sdk

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

### App Interaction
After launching, you can interact with the pallet using the following link: [Polkadot.js Explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9920#/explorer).
