# Deployment status

| Network | Ditto Stellar vault | Job registry | Release evidence |
| --- | --- | --- | --- |
| Stellar testnet | Not deployed | Not deployed | No transaction hashes or test results published |
| Stellar mainnet | Not deployed | Not deployed | Deposits closed; independent review required |

Existing Ditto deployments on other networks are listed in [prior work](evidence.md). They are not Stellar deployments.

Before a deployment is announced, publish a manifest containing:

- Network/passphrase, release tag, source commit and contract code hashes.
- Vault, registry, USDC asset contract and approved pool contract IDs.
- Admin account, signer thresholds and executor accounts; no private keys.
- Public policy parameters, caps, supported jobs and pause/upgrade permissions.
- For Base onboarding: official Circle contract addresses, domains and recipient configuration.
- External review report and resolved-finding references for mainnet.
- Deposit, allocation, cancellation and redemption transaction hashes.

Operators, SDK consumers and the application must use the same verified manifest. Values are intentionally absent until actual deployments exist.
