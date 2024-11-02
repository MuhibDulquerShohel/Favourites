# Favourite Program Using Anchor Framework

This repository contains a Solana smart contract implemented using the Anchor framework. The `favourite` program allows users to store personalized data such as their favourite number, color, and hobbies.

## Features

- **User-Specific Data Storage**: Users can record their favourite number, color, and hobbies.
- **Dynamic Space Allocation**: Uses `#[derive(InitSpace)]` for efficient account space management.
- **Seed-Based Account Derivation**: Ensures data is securely linked to user public keys.
- **Conditional Account Initialization**: Supports `init_if_needed` for seamless account creation.
- **Secure User Authentication**: Authorizes interactions with `Signer` validation.
- **On-Chain Logging**: Provides transparent data recording with the `msg!()` macro.

## Program Structure

### `set_favourites` Function

- **Inputs**:
  - `number` (`u64`): User's favourite number.
  - `color` (`String`): User's favourite color (max 50 characters).
  - `hobbies` (`Vec<String>`): List of up to 5 hobbies (each max 50 characters).
- **Functionality**: Sets and logs the user's data, saving it to the associated on-chain account.

### `Favourites` Struct

Defines the data schema:

- `number`: User's favourite number.
- `color`: A string with a 50-character limit.
- `hobbies`: A vector allowing up to 5 strings, each with a 50-character limit.

### `SetFavourites` Context

Manages the account setup and validation:

- Ensures the user is a `Signer`.
- Initializes the `Favourites` account if needed, with a derived seed `[b"favourites", user.key().as_ref()]`.

## Installation and Usage

1. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/favourite-program.git
   cd favourite-program
   ```

2. **Build and deploy**:
   Follow Anchor's standard build and deployment commands:

   ```bash
   anchor build
   anchor deploy
   ```

3. **Run the client**:
   Use a client script to interact with the deployed program and set user data.

## Example Command

```rust
// Example interaction with the set_favourites function
let tx = program
    .request()
    .accounts(SetFavourites {
        user: user_pubkey,
        favourites: favourites_account,
        system_program: system_program,
    })
    .args(set_favourites(42, "blue".to_string(), vec!["reading".to_string(), "coding".to_string()]))
    .send()?;
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## Contributing

Contributions are welcome! Please open issues and submit pull requests for improvements or additional features.
