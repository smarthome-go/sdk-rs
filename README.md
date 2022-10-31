# Smarthome SDK (Rust rewrite)

This is the new and improved version of the [old SDK](https://github.com/smarthome-go/sdk).

A Rust create which makes communication to a Smarthome server simple. It can be
seen as a API wrapper for some commonly-used functions of the Smarthome server's
API.

## Usage

```bash
cargo add smarthome-sdk-rs
```

```rust
use smarthome_sdk_rs::{Auth, Client};

#[tokio::main]
async fn main() {
    // Create a new Smarthome client
    let client = Client::new(
        "http://localhost:8082",
        Auth::QueryToken("b67f2f5c7f2e6795d9f9b55678db7579".to_string()),
    )
    .await
    .unwrap();

    // Do something with the client
    // This will turn on the finctional desk lamp
    client.set_power("desk_lamp", true).await.unwrap();
}
```
