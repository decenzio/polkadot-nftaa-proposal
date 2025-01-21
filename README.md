# NFTAA Proposal

This is proposal for the NFTAA pallet

## How to run?

For dockerized version you need to build image first using:

```sh
docker build --platform linux/x86_64 -t polkadot-sdk-image:latest .
```

And than run zombienet instance with:

```sh
docker run --platform linux/x86_64 -p 9910-9913:9910-9913 -p 9920-9921:9920-9921 --rm -it polkadot-sdk-image:latest
```