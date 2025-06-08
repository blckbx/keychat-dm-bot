## Encrypted Nostr Private Messenger

A simple Rust command-line app to send encrypted private messages over the Nostr protocol using the [nostr-sdk](https://github.com/rust-nostr/nostr-sdk). The app leverages NIP-17 (private messaging) and NIP-44 (encryption) to securely deliver messages to any Nostr public key via a relay.

---

**Features**

- End-to-end encrypted private messaging using Nostr standards
- Easy command-line interface: send a message with a single command
- Environment-based configuration for secret key, recipient, and relay
- Built with async Rust and [nostr-sdk](https://github.com/rust-nostr/nostr-sdk)

---

## Usage

### 1. Prerequisites

- Rust (edition 2021 or later)
- A `.env` file with your bot's secret key, recipient's npub, and relay URL


### 2. Install Dependencies

```sh
cargo build --release
```


### 3. Prepare Your `.env` File

Create a `.env` file in the project root with the following variables:

```env
BOT_SECKEY=your_nsec_or_hex_secret_key
RECEIVER_NPUB=recipient_npub1...
RELAY_URL=wss://your.nostr.relay
```


### 4. Send a Message

```sh
cargo run --release -- "Hello, this is my message"
```

**Example Output:**

```
Recipient's Public Key: npub1...
Connecting to relay: wss://your.nostr.relay...
Connection successful!

Sending message: "Hello, this is my message"

Successfully sent encrypted private message!
Event ID: 0123456789abcdef...
```


---

## How It Works

- Loads configuration from `.env` for keys and relay details
- Parses the recipient's npub and your secret key
- Connects to the specified Nostr relay
- Encrypts the message using NIP-44 (XSalsa20-Poly1305)
- Sends the encrypted event privately to the recipient

---

## Example

```sh
cargo run --release -- "gm nostr!"
```
or
```sh
./keychat-dm-bot "gm nostr!"
```


---

## Configuration

| Variable | Description |
| :-- | :-- |
| BOT_SECKEY | Your Nostr secret key (nsec or hex) |
| RECEIVER_NPUB | Recipient's public key (npub...) |
| RELAY_URL | WebSocket URL of the Nostr relay |


---

## Security

- Messages are encrypted end-to-end using NIP-44.
- Secret keys and configuration should be kept private and never committed to source control.

---

## License

MIT

---

## Credits

- [nostr-sdk](https://github.com/rust-nostr/nostr-sdk)
- Inspired by the Nostr protocol and NIP-17/NIP-44 standards