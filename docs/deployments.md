# Deployment status

| Network | Ditto Stellar vault | Job registry | Release evidence |
| --- | --- | --- | --- |
| Stellar testnet: minimal proof | DepositProof deployed; full Earn not deployed | Not deployed | [Real deposit/withdraw evidence](../prototype/README.md), 6 passing unit tests |
| Stellar mainnet | Not deployed | Not deployed | Deposits closed; independent review required |

Existing Ditto deployments on other networks are listed in [prior work](evidence.md). They are not Stellar deployments.

Before a deployment is announced, publish a manifest containing:

- Network/passphrase, release tag, source commit and contract code hashes.
- Vault, registry, USDC asset contract and approved strategy/adapter contract IDs, underlying versions and receipt-token identifiers.
- Admin account, signer thresholds and executor accounts; no private keys.
- Public policy parameters, caps, supported jobs and pause/upgrade permissions.
- For Base/Ethereum onboarding: official Circle contract addresses, domains and recipient configuration.
- For Solana onboarding: 1Click source/destination asset identifiers, recipient validation and refund configuration.
- External review report and resolved-finding references for mainnet.
- Deposit, allocation, cancellation and redemption transaction hashes for both strategy adapters and all supported funding routes.

Operators, SDK consumers and the application must use the same verified manifest. The minimal proof has its own [manifest](evidence/testnet-proof-2026-10-27.json). The full Earn/registry manifest will be published with those deployments.
