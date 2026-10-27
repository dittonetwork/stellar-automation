# Prior work

Ditto's existing products provide delivery experience. None of the figures below represents Stellar users, Stellar TVL or security already available to this build.

## Mezo and Spark

The Ditto MUSD Savings Vault accepts MUSD on Mezo and issues dMUSD. Ditto coordinates swaps, bridging and allocation to Spark Savings on Ethereum, with redemption back to Mezo.

- [Mezo product documentation](https://mezo.org/docs/users/mezo-earn/vaults/ditto-musd-savings-vault/)
- [Mezo vault](https://mezo.org/earn/vaults/0x9171Cb787C3fEEbBC808D07CaB62B1d789CE14F8)
- [Ditto announcement](https://x.com/Ditto_Network/status/2104956318494994853)
- [Public Mezo operator distribution](https://github.com/dittonetwork/mezo-stable-yield-operator)

On **27 October 2026, 08:25 UTC**, the vault reported **105,156.32653730386 MUSD** through `totalAssets()` at Mezo mainnet block **12,798,527**. The [dated RPC snapshot](evidence/mezo-snapshot-2026-10-27.json) includes the block hash, asset address, decimals and raw responses. This is contract-reported accounting, not an independent reconciliation of cross-chain backing. It is not proof of the historical team-reported 101,000 MUSD first-24-hour figure. No unique-user count is claimed.

To reproduce, call `eth_call` on the vault at block `0xc34a3f` with `totalAssets()` selector `0x01e1d114`; read `asset()` (`0x38d52e0f`) and its `decimals()` (`0x313ce567`). The snapshot records the endpoint and exact results. Use the JSON block number to derive the hex block tag if needed.

## Operator infrastructure

Ditto operates EVM automation infrastructure using EigenLayer and Symbiotic restaking. Team records report approximately **$600 million in historical peak restaked collateral** and **350,000+ operator simulations**. Collateral is separate from vault deposits; simulations are not user transactions. Stellar execution controls are new work in this repository.

- [Symbiotic network profile](https://resources.symbiotic.fi/network-highlight-ditto)
- [Existing EVM operator code](https://github.com/dittonetwork/ditto-runner-node)
- [Existing SDK](https://github.com/dittonetwork/sdk-js)

These repositories are linked as prior work, not copied or relicensed here. Historical metrics should be accompanied by dated exports when used as formal grant evidence.

## Stellar pre-award proof

A minimal deposit/withdraw proof using native testnet XLM was completed before the award at Ditto's own expense. Its source, tests and transaction evidence are public. This proof is excluded from the grant budget; it does not implement USDC integration, yield shares, pool accounting, operators or CCTP. [Source, tests and transaction evidence](../prototype/README.md).
