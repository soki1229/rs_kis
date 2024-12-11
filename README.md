# rs_kis

rs_kis is a Rust library for interacting with the Korea Investment & Securities (KIS) API. It aims to provide a convenient way to access various financial data and perform trading operations through the KIS platform.

**IMPORTANT: This project is currently under active development and is not fully implemented. Many features are NOT SUPPORTED YET.**

## Current Status

This library is in its early stages of development. The following features are planned but not yet fully implemented:

- Authentication with the KIS API
- Real-time stock price information
- Order placement and management
- Account balance and portfolio information
- Historical price data retrieval

Please check the project's issues and pull requests for the most up-to-date information on development progress.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rs_kis = "0.1.0"
```

## Usage

**NOTE: The following example is for illustration purposes only. Most functionalities are NOT SUPPORTED YET.**

```rust
use rs_kis::KisClient;

#[tokio::main]
async fn main() {
    let client = KisClient::new("your_app_key", "your_app_secret", "your_account_number");
    
    // Authenticate (NOT SUPPORTED YET)
    client.authenticate().await.unwrap();
    
    // Get real-time stock price (NOT SUPPORTED YET)
    let price = client.get_stock_price("005930").await.unwrap();
    println!("Current price of Samsung Electronics: {}", price);
}
```

## Configuration

Before using the library, you will need to set up your KIS API credentials. You can obtain these from the KIS developer portal. Detailed instructions on how to configure the library will be provided as development progresses.

## Contributing

Contributions are welcome! As this project is in its early stages, there are many opportunities to contribute. Please feel free to submit issues, feature requests, or pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This library is not officially associated with Korea Investment & Securities. It is an independent project in early development stages. Use at your own risk.

## Roadmap

### Phase 1: Initial Setup and Authentication
- [x] Set up project structure and initial dependencies
- [x] Implement user authentication with KIS API
- [x] Obtain access tokens for API calls

### Phase 2: Core Features Implementation
- [ ] **Account Management**
  - Implement functionality to retrieve account balance and details.
  - Support for multiple account management.

- [ ] **Stock Trading Operations**
  - Implement order placement (buy/sell) for domestic stocks.
  - Implement order modification and cancellation.
  - Add functionality to check order status.

- [ ] **Market Data Retrieval**
  - Implement real-time stock price retrieval using WebSocket.
  - Support for historical price data retrieval.
  - Implement functionality to check available buying power.

### Phase 3: Advanced Features
- [ ] **Portfolio Management**
  - Implement portfolio overview and performance tracking.
  - Add features to analyze historical performance.

- [ ] **Notifications and Alerts**
  - Implement a notification system for trade confirmations and market alerts.
  
- [x] **WebSocket Integration**
  - Establish a WebSocket connection for real-time updates on stock prices and trading activity.

### Phase 4: Testing and Documentation
- [ ] Comprehensive testing of all implemented features.
- [ ] Create detailed documentation for API usage.
- [ ] Provide examples and sample code for common use cases.

### Phase 5: Community Feedback and Iteration
- [ ] Gather feedback from users on implemented features.
- [ ] Prioritize feature requests and improvements based on user input.
- [ ] Regularly update the library based on KIS API changes.

### Future Considerations
- Explore integration with additional financial services or APIs.
- Consider implementing a GUI or web interface for easier interaction with the library.

**Note:** The roadmap is subject to change as development progresses. Please check the repository regularly for updates on feature implementation and project status. Your contributions, feedback, and suggestions are welcome!

Please note that this roadmap is subject to change as the project evolves.