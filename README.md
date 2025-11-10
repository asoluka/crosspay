# CrossPay - Decentralized Remittance Platform

**A Solana-based remittance and micro-payments platform addressing high-cost cross-border payments to Africa.**

![Solana](https://img.shields.io/badge/Solana-Blockchain-blueviolet)
![Anchor](https://img.shields.io/badge/Anchor-0.32.1-blue)
![License](https://img.shields.io/badge/License-MIT-green)

---

## 🎯 Overview

CrossPay enables diaspora Africans and global clients to send funds instantly to freelancers, families, and small businesses using stablecoins like USDC on Solana. By leveraging blockchain technology and smart contracts, CrossPay provides:

- **Speed**: Instant transfers with near-zero settlement time
- **Low Cost**: ~0.5% platform fee vs 5-8% traditional remittance
- **Transparency**: On-chain audit trail with programmable logic
- **Accessibility**: Mobile money integration for local fiat access

---

## 🚀 Key Features

### For Senders (Diaspora)
- KYC-verified profiles for compliance
- Instant USDC transfers to any wallet
- Real-time transaction tracking
- Support for multiple beneficiaries

### For Receivers (Freelancers/Families)
- Receive payments in stablecoins (USDC)
- Cash out to local currency via liquidity providers
- Multiple payout methods (Mobile Money, Bank, Cash)
- Track incoming payments and balances

### For Liquidity Providers (P2P Agents)
- Register as local fiat on/off-ramp
- Set competitive exchange rates
- Earn fees on cash-out transactions
- Build trust score through successful transactions

---

## 📋 Architecture

### State Accounts (PDAs)

| Account | Size | Description |
|---------|------|-------------|
| **UserProfile** | 108 bytes | User identity, KYC status, transaction totals |
| **TransferRequest** | 140 bytes | Remittance transaction details and status |
| **WithdrawalRequest** | 143 bytes | Fiat cash-out request with LP selection |
| **LiquidityProvider** | 138 bytes | P2P agent profile with trust scoring |

### Instructions (9 Total)

#### User Management
1. `initialize_user` - Create user profile
2. `update_kyc_status` - Update KYC verification

#### Transfer Flow
3. `initiate_transfer` - Create transfer request
4. `confirm_transfer` - Execute token transfer

#### LP Management
5. `register_liquidity_provider` - Register as LP
6. `update_provider_availability` - Update liquidity/status

#### Withdrawal Flow
7. `request_withdrawal` - Request cash-out
8. `select_provider` - Choose LP for withdrawal
9. `finalize_withdrawal` - Complete withdrawal

---

## 🛠️ Tech Stack

- **Blockchain**: Solana
- **Framework**: Anchor 0.32.1
- **Language**: Rust
- **Token Standard**: SPL Token (Token-2022 compatible)
- **Testing**: TypeScript + Mocha

---

## 📦 Installation & Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.1
avm use 0.32.1

# Install Node.js dependencies
yarn install
```

### Build & Test

```bash
# Build the program
anchor build

# Get program ID
solana-keygen pubkey target/deploy/crosspay-keypair.json

# Update program ID in lib.rs and Anchor.toml
anchor keys sync

# Run tests
anchor test
```

### Deploy

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet-beta (production)
anchor deploy --provider.cluster mainnet-beta
```

---

## 🔄 User Flows

### Flow 1: Remittance (Sender → Receiver)

```
1. Sender creates profile → initialize_user(role: Sender)
2. Sender completes KYC → update_kyc_status(verified: true)
3. Sender initiates transfer → initiate_transfer(100 USDC, receiver_key)
4. Sender confirms transfer → confirm_transfer()
✅ Result: 100 USDC transferred to receiver
```

### Flow 2: Cash-Out (Receiver → Fiat via LP)

```
1. Receiver creates profile → initialize_user(role: Receiver)
2. LP registers → register_liquidity_provider(location, rate)
3. LP updates liquidity → update_provider_availability(1000 USDC, active)
4. Receiver requests withdrawal → request_withdrawal(50 USDC, mobile_money)
5. Receiver selects LP → select_provider(lp_key)
6. Both parties finalize → finalize_withdrawal()
✅ Result: Receiver gets NGN, LP gets 50 USDC
```

---

## 🔐 Security Features

- **KYC Verification**: Required for senders to prevent fraud
- **PDA-based Access Control**: Only authorities can modify accounts
- **Status-based State Machines**: Prevents invalid state transitions
- **Balance Validation**: Checks before all transfers
- **Trust Scoring**: LP reputation system (out of 10000)

---

## 💰 Fee Structure

| Transaction Type | Fee | Notes |
|------------------|-----|-------|
| Transfer (Sender → Receiver) | 0.5% | Platform fee on amount |
| Withdrawal (Receiver → LP) | Variable | Set by LP (competitive market) |
| LP Registration | Free | No upfront cost |

---

## 📊 Example Transaction Costs

| Amount | Traditional | CrossPay | Savings |
|--------|-------------|----------|---------|
| $100 | $8 (8%) | $0.50 (0.5%) | $7.50 (93.75%) |
| $500 | $40 (8%) | $2.50 (0.5%) | $37.50 (93.75%) |
| $1000 | $80 (8%) | $5.00 (0.5%) | $75.00 (93.75%) |

---

## 🧪 Testing

### Run All Tests
```bash
anchor test
```

### Test Coverage
- ✅ User profile initialization
- ✅ KYC updates
- ✅ Transfer initiation and execution
- ✅ LP registration and availability
- ✅ Withdrawal request and completion
- ✅ Token balance verification
- ✅ State transition validation

### Expected Output
```
User Management
  ✔ Initializes sender user profile (482ms)
  ✔ Initializes receiver user profile (478ms)
  ✔ Updates KYC status for sender (472ms)
Transfer Flow
  ✔ Initiates a transfer (492ms)
  ✔ Confirms and executes the transfer - 100 USDC (481ms)
Liquidity Provider Management
  ✔ Registers a liquidity provider (469ms)
  ✔ Updates LP availability - 1000 USDC liquidity (480ms)
Withdrawal Flow
  ✔ Requests a withdrawal - 50 USDC (484ms)
  ✔ Selects a liquidity provider (489ms)
  ✔ Finalizes the withdrawal - 50 USDC to LP (482ms)
Complete Flow Summary

📊 Final Balances:
Sender: [amount] USDC
Receiver: [amount] USDC
LP: [amount] USDC

✅ All flows completed successfully!
      ✔ Shows final balances


  11 passing (10s)
```

---

## 📁 Project Structure

```
crosspay/
├── programs/crosspay/src/
│   ├── lib.rs                          # Program entry point
│   ├── errors.rs                       # Custom error codes
│   ├── constants.rs                    # Seeds, defaults, helpers
│   ├── state/
│   │   ├── user_profile.rs            # User state
│   │   ├── transfer_request.rs        # Transfer state
│   │   ├── withdrawal_request.rs      # Withdrawal state
│   │   └── liquidity_provider.rs      # LP state
│   └── instructions/
│       ├── initialize_user.rs         # User management
│       ├── initiate_transfer.rs       # Transfer flow
│       ├── confirm_transfer.rs
│       ├── request_withdrawal.rs      # Withdrawal flow
│       ├── select_provider.rs
│       ├── finalize_withdrawal.rs
│       └── register_liquidity_provider.rs # LP management
├── tests/crosspay.ts                  # Integration tests
├── Anchor.toml                        # Anchor config
└── Cargo.toml                         # Rust dependencies
```

---

## 🌍 Target Markets

1. **Diaspora Africans** - US, UK, Canada sending remittances
2. **African Freelancers** - Remote workers earning in USD/EUR
3. **Small Businesses** - Cross-border traders
4. **Mobile-First Users** - High mobile money penetration
5. **Crypto-Native Youth** - Tech hubs (Lagos, Nairobi, Accra)

---

## 🎯 Roadmap

### Phase 1: MVP (Current)
- ✅ Core transfer functionality
- ✅ LP marketplace
- ✅ Withdrawal system

### Phase 2: Enhancements
- [ ] Escrow for disputed transactions
- [ ] Multi-signature support
- [ ] Automated LP matching algorithm
- [ ] Mobile app (React Native)

### Phase 3: Expansion
- [ ] Multi-chain support (Ethereum, Polygon)
- [ ] Fiat on-ramp integrations (MoonPay, Ramp)
- [ ] Mobile money API integrations (Flutterwave, Paystack)
- [ ] Advanced analytics dashboard

### Phase 4: Scale
- [ ] DAO governance
- [ ] Staking mechanism
- [ ] Loyalty rewards program
- [ ] Enterprise API

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👥 Team

- **Asoluka Tochukwu Austin** - Founder & Lead Developer
- Building with guidance from Turbin3 program

---

## 📞 Contact & Support

- **Email**: [stracool9@gmail.com](stracool9@gmail.com)
- **Twitter**: [@thesirtochi](https://x.com/thesirtochi)
- **Discord**: [sleekCodes]
- **Documentation**: [docs](docs.crosspay.io)

---

## 🙏 Acknowledgments

- Solana Foundation for the robust blockchain infrastructure
- Anchor framework for excellent developer experience
- Turbin3 for educational support and guidance
- African crypto community for inspiration and feedback

---

## ⚠️ Disclaimer

This is educational/experimental software. Use at your own risk. Always conduct thorough audits before deploying to production. Not financial advice.

---

**Built with ❤️ for Africa �