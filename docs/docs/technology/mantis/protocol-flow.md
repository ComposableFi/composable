# Protocol Flow
A summary of the MANTIS protocol flow in different scenarios is below, with further details found in subsequent sections of this documentation:

## 1. Intent Submission:
- **User-Driven Transactions**: Users specify their transaction requirements, typically involving an exchange of a certain amount of one cryptocurrency (Token A) for another (Token B).
- **Assisted Order Formulation**: MANTIS assists in setting up order limits; in the example of exchanging A for B, MANTIS provides suggestions for the exchange amount of Token B. The exchange rate will not be less than the user-defined A/B ratio.
- **Confirmation and Blockchain Registration**: Users review, confirm, and sign their transaction details for blockchain recording.
- **Timeout vs. Price Limits**: A balance between price limits and matching times is maintained, with tighter limits possibly leading to longer wait times for order matching.

## 2. Order Execution Observation:
- **Status Monitoring**: Users can track the status of their orders post-placement.
- **Possible Outcomes**: Orders may be fully executed, partially filled, canceled, or timed out.
- **Handling Partial Fills**: Partially filled orders result in users receiving a portion of the requested amount, with the remainder being canceled or expiring based on the order settings.

## 3a. Single-Chain Execution Scenario:
- **Efficient Execution**: The platform swiftly matches orders in a single transaction block for prompt fulfillment.
- **Batch Auctions**: Batch Auctions process multiple orders simultaneously, maximizing the product of exchanged amounts (A * B) for efficient matching.

### 3b. Order Pricing:

- **Dynamic Price Matching**: The platform matches orders to achieve optimal trading volume without violating user-set limits.
- **Execution at Optimal Prices**: Execution occurs at a price that maximizes volume, ensuring efficiency.

## 4. Cross-Chain Execution Scenario:
- **Multi-Chain Execution**: Certain orders are executed using liquidity pools across multiple blockchain networks, involving several blocks and chains.
- **Cross-Chain Virtual Machine (CVM) Program**: The CVM facilitates these transactions, ensuring efficient multi-chain swaps.
- **Monitoring Interface**: A detailed interface provides real-time updates for multi-chain transactions.
- **Cross-Chain Transfers**: This includes straightforward cross-chain transfers.

The above components are displayed in some form in the following architecture diagram of Composable’s Cosmos chain:

![architecture](../mantis/components.png)