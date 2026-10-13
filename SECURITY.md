# Security Policy

## Reporting Security Issues

We take the security of SplitPay's smart contracts and backend infrastructure seriously. If you discover a vulnerability, please report it privately.

### Responsible Disclosure Guidelines
- **Do NOT** file public GitHub issues for security vulnerabilities.
- Send vulnerability details to `security@splitpay.app` or initiate a private GitHub Security Advisory.
- Include a technical description of the issue, affected methods (e.g., Soroban contract functions or API endpoints), and reproduction steps.
- Maintainers will acknowledge and begin investigation within 48 hours.

### Scope
- Soroban smart contract logic (`contracts/src/lib.rs`): multi-session balance math, double-payment prevention, token settlement integrity.
- NestJS API (`src/`): payment recording, validation of transaction memos, state reconciliation.
