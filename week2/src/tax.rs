use crate::{Product, ProductType};

pub const FOOD_TAX_RATE: f64 = 0.05;
pub const ELECTRONICS_TAX_RATE: f64 = 0.10;
pub const BOOKS_TAX_RATE: f64 = 0.0;

pub struct TaxCalculation {
    product_name: String,
    original_price: f64,
    tax_rate: f64,
    tax_amount: f64,
    final_price: f64,
}

pub fn calculate_tax(product: &Product) -> TaxCalculation {
    let tax_rate: f64 = match product.product_type {
        ProductType::Food => FOOD_TAX_RATE,
        ProductType::Electronics => ELECTRONICS_TAX_RATE,
        ProductType::Books => BOOKS_TAX_RATE,
    };

    let tax_amount: f64 = product.price * tax_rate;

    TaxCalculation {
        product_name: product.name.clone(),
        original_price: product.price,
        tax_rate,
        tax_amount,
        final_price: product.price + tax_amount,
    }
}

pub fn print_tax_results(results: &Vec<TaxCalculation>) {
    for result in results {
        println!("---");
        println!("Product: {}", result.product_name);
        println!("Original Price: ${:.2}", result.original_price);
        println!("Tax Rate: {:.2}%", result.tax_rate * 100.0);
        println!("Tax Amount: ${:.2}", result.tax_amount);
        println!("Final Price: ${:.2}", result.final_price);
        println!("---");
    }
}