mod product;
mod tax;

use product::{Product, ProductType};

fn main() {
    let book1 = Product {
        name: "Learning to learn".to_string(),
        price: 30.00,
        product_type: ProductType::Books,
    };

    let smartphone1 = Product {
        name: "iPhone 17".to_string(),
        price: 800.00,
        product_type: ProductType::Electronics,
    };

    let food1 = Product {
        name: "Fried Rice".to_string(),
        price: 10.00,
        product_type: ProductType::Food,
    };

    let products: [Product; 3] = [book1, smartphone1, food1];

    let mut results: Vec<tax::TaxCalculation> = Vec::new();
    for product in &products {
        let tax_result: tax::TaxCalculation = tax::calculate_tax(product);
        results.push(tax_result);
    }

    tax::print_tax_results(&results);
}
