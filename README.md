# Rust Web Service for Currency Exchange Rates

This project is a Rust-based web service that serves currency exchange rates in JSON format. It provides endpoints to retrieve today's and yesterday's exchange rates for various currencies.

## Project Structure

```
rust-web-service
├── src
│   ├── main.rs          # Entry point of the application
│   ├── routes
│   │   └── mod.rs      # Routing logic for currency endpoints
│   ├── handlers
│   │   └── currency.rs  # Logic for fetching and returning currency rates
│   └── utils
│       └── fetch.rs     # Utility functions for fetching exchange rate data
├── Cargo.toml           # Configuration file for the Rust project
└── README.md            # Documentation for the project
```

## Setup Instructions

1. **Clone the repository:**

   ```
   git clone <repository-url>
   cd rust-web-service
   ```

2. **Build the project:**

   ```
   cargo build
   ```

3. **Run the web service:**

   ```
   cargo run
   ```

## Usage

- To get today's exchange rate for a specific currency:

  ```
  GET /today/{CURRENCY_CODE}.json
  ```

- To get yesterday's exchange rate for a specific currency:

  ```
  GET /yesterday/{CURRENCY_CODE}.json
  ```

Replace `{CURRENCY_CODE}` with the appropriate currency code (e.g., USD, EUR).

## Dependencies

This project uses several dependencies specified in the `Cargo.toml` file. Make sure to check it for any additional setup required.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
