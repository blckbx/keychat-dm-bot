use nostr_sdk::prelude::*;
use std::env;
use dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    // Check if message argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <message>", args[0]);
        eprintln!("Example: {} \"Hello, this is my message\"", args[0]);
        std::process::exit(1);
    }
    
    let message_content = &args[1];

    println!("{:?}", dotenv::dotenv().ok());

    // Load from environment variables
    let bot_seckey = env::var("BOT_SECKEY")
        .expect("BOT_SECKEY must be set in .env file");
    let receiver_npub = env::var("RECEIVER_NPUB")
        .expect("RECEIVER_NPUB must be set in .env file");
    let relay_url = env::var("RELAY_URL")
        .expect("RELAY_URL must be set in .env file");

    // To use an existing secret key (nsec or hex):
    let my_keys = Keys::parse(bot_seckey)?;

    // Parse the Recipient's Public Key
    // Convert the recipient's npub string into a PublicKey object.
    let recipient_pubkey = PublicKey::from_bech32(receiver_npub)?;
    println!("Recipient's Public Key: {}", recipient_pubkey.to_bech32()?);

    // Initialize the Nostr Client
    let client = Client::new(&my_keys);
    client.add_relay(relay_url.clone()).await?;
    
    // Connect to the Relay
    println!("\nConnecting to relay: {}...", relay_url);
    client.connect().await;
    println!("Connection successful!");

    // Encrypt and Send the Private Message
    println!("\nSending message: \"{}\"", message_content);
    
    // The `send_private_msg` method handles all the necessary encryption steps
    // according to NIP-17, which uses NIP-44 for encryption.
    let event_id = client.send_private_msg(recipient_pubkey, message_content, None).await?;
    
    println!("\nSuccessfully sent encrypted private message!");
    println!("Event ID: {}", event_id.to_hex());

    // Shutdown the client
    client.shutdown().await?;

    Ok(())
}
