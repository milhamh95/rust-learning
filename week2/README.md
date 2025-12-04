# Rust Tax Calculator

A simple tax calculator built in Rust that demonstrates the use of structs, enums, and constants. This project calculates tax for different product categories (Food, Electronics, Books) and provides detailed tax breakdowns.

## Project Description

This tax calculator:
- Uses **enums** to define product types (Food, Electronics, Books)
- Uses **structs** to organize product and tax calculation data
- Uses **constants** for tax rates (Food: 5%, Electronics: 10%, Books: 0%)
- Calculates tax for multiple products and displays detailed breakdowns

### Features
- Tax calculation for three product categories
- Detailed tax breakdown showing original price, tax rate, tax amount, and final price
- Support for multiple products in a single calculation
- Clean, idiomatic Rust code demonstrating ownership and borrowing concepts

## How to Run This Project

### Prerequisites
- [Rust](https://rustup.rs/) installed on your system

### Steps

1. **Run the program**:
   ```bash
   cargo run
   ```

### Example Output
```
Product: Learning to learn
Original Price: $30.00
Tax Rate: 0.00%
Tax Amount: $0.00
Final Price: $30.00
---
Product: iPhone 17
Original Price: $800.00
Tax Rate: 10.00%
Tax Amount: $80.00
Final Price: $880.00
---
Product: Fried Rice
Original Price: $10.00
Tax Rate: 5.00%
Tax Amount: $0.50
Final Price: $10.50
---
```

## Project Structure
```
src/
└── main.rs          # Main program with tax calculation logic
Cargo.toml           # Project configuration
README.md            # This file
PRD.md               # Product Requirements Document
rust-setup-guide.md  # Rust setup instructions
```