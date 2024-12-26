# Decentralized Digital Time Capsule System

A decentralized application built on the Internet Computer platform that allows users to create, store, and manage digital time capsules with customizable unlock conditions and access controls.

## Features

### Content Management
- Multiple content types support:
  - Plain text messages
  - Encrypted messages with public key encryption
  - IPFS media references
  - Multi-part messages with various content types

### Access Control
- Public access for community-wide capsules
- Private access with specified viewers
- Conditional access mechanisms:
  - Token-gated access
  - Geographic location verification
  - Quiz/Challenge completion
  - Smart contract conditions

### Cultural & Historical Features
- Cultural significance tagging
- Location-based discovery
- Historical context preservation
- Metadata management
- Geographical coordinates binding

## Getting Started

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
npm install -g dfx
```

### Installation
1. Clone the repository:
```bash
git clone https://github.com/n96311/decentralized-digital-time-capsule-system.git
cd decentralized-digital-time-capsule-system
```

2. Start the local Internet Computer replica:
```bash
dfx start --clean --background
```

3. Deploy the canister:
```bash
dfx deploy
```

## Usage

### Creating a Time Capsule

```rust
let capsule_payload = CreateCapsulePayload {
    content: CapsuleContent::Text("Future message".to_string()),
    unlock_date: time() + 31_536_000, // One year from now
    access_control: AccessControl::Public,
    metadata: CapsuleMetadata {
        title: "My First Capsule".to_string(),
        description: "A message to the future".to_string(),
        tags: vec!["personal".to_string()],
        location: None,
        cultural_significance: None,
    },
};

let result = create_time_capsule(capsule_payload);
```

### Retrieving a Capsule

```rust
match get_capsule(capsule_id) {
    Ok(capsule) => {
        // Process unlocked capsule
    },
    Err(e) => {
        // Handle error (e.g., "Capsule is still sealed")
    }
}
```

### Finding Location-Based Capsules

```rust
let nearby_capsules = get_capsules_by_location(
    latitude,
    longitude,
    radius_km
);
```

## API Reference

### Core Functions

#### `create_time_capsule`
Creates a new time capsule with specified content and conditions.

```rust
#[ic_cdk::update]
fn create_time_capsule(payload: CreateCapsulePayload) -> Result<TimeCapsule, String>
```

#### `get_capsule`
Retrieves a capsule if unlock conditions are met.

```rust
#[ic_cdk::query]
fn get_capsule(capsule_id: u64) -> Result<TimeCapsule, String>
```

#### `get_public_capsules`
Retrieves all publicly accessible and unlocked capsules.

```rust
#[ic_cdk::query]
fn get_public_capsules() -> Vec<TimeCapsule>
```

#### `get_capsules_by_location`
Finds capsules within a specified radius of given coordinates.

```rust
#[ic_cdk::query]
fn get_capsules_by_location(latitude: f64, longitude: f64, radius_km: f64) -> Vec<TimeCapsule>
```

## Data Structures

### TimeCapsule
```rust
struct TimeCapsule {
    id: u64,
    creator: String,
    creation_date: u64,
    unlock_date: u64,
    content: CapsuleContent,
    access_control: AccessControl,
    metadata: CapsuleMetadata,
    status: CapsuleStatus,
}
```

### CapsuleContent
```rust
enum CapsuleContent {
    Text(String),
    EncryptedMessage { content: Vec<u8>, public_key: String },
    MediaReference { ipfs_hash: String, media_type: String },
    MultipartMessage { parts: Vec<CapsuleContent>, title: String },
}
```

## Security Considerations

- All capsule content is stored on-chain and is immutable once created
- Encrypted messages should use strong encryption before submission
- Access control mechanisms are enforced at the smart contract level
- Geographic verification requires trusted oracle integration
- Token-gated access requires valid token ownership verification

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Internet Computer Platform Documentation
- Rust Stable Structures Library
- IPFS Documentation
- Geographic Distance Calculation Formula Sources

## Future Enhancements

- Integration with social platforms
- AR/VR viewing experiences
- Collaborative capsule creation
- NFT-gated access implementation
- Treasure hunt game mechanics
- Educational institution partnerships
- Automated cultural preservation programs