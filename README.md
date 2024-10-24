![Logo](https://cdn.prod.website-files.com/6595b2282ea917577755d3a5/6595bb9290625dfff5df3f7e_Logo%20-%20Color.svg)

# Normal Wormhole Updater

The Normal Wormhole Updater uses [Wormhole Queries](https://wormhole.com/queries/) to bring cross and off-chain data to the [Normal Protocol](https://github.com/normalfinance/normal-v1) on Solana.

## Features

- 🤑 Asset prices unsupported by Solana oracles
- 🏦 Real-time lending, staking, and LP yield rates
- 🖼️ NFT floor prices and metadata
- ❓ Prediction market outcomes
- and more...

## Documentation

Learn more about developing with Queries in [the docs](https://docs.wormhole.com/wormhole/queries/getting-started).

Read more on how the Normal Protocol uses this Wormhole Updater in [our docs](https://docs.normalfinance.io).

### Accounts

- [GuardianSignatures](programs/updater/src/state/guardian_signatures.rs) stores unverified guardian signatures for subsequent verification. These are created with `post_signatures` in service of verifying a root via Queries and closed when that root is verified with `verify_query` or can be explicitly closed with `close_signatures` by the initial payer.

### Instructions

- [post_signatures](programs/updater/src/instructions/post_signatures.rs) posts unverified guardian signatures for verification during `update_root_with_query`.
- [verify_query](programs/updater/src/instructions/verify_query.rs) with a Query response and `GuardianSignatures` account, verifies the signatures against an active guardian set and logs the Query response. This is where you would add additional verification relevant to your use case and process the result.
- [close_signatures](programs/updater/src/instructions/close_signatures.rs) allows the initial payer to close a `GuardianSignatures` account in case the query was invalid.

## Testing

```bash
anchor test
```

## Building

### Wormhole Testnet / Solana Devnet

```bash
anchor build -- --no-default-features --features testnet
```

### Mainnet

```bash
anchor build
```

## Authors

- [@wormholelabs-xyz](https://github.com/wormholelabs-xyz)
- [@evan-grey](https://github.com/evan-gray)
- [@jblewnormal](https://www.github.com/jblewnormal)

## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.

## Feedback

If you have any feedback, please reach out to us at hello@normalfinance.io

## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
