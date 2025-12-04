enum ProductType {
    Food,
    Electronics,
    Books,
}

struct Product {
    name: String,
    price: f64,
    product_type: ProductType,
}

struct TaxCalculation {
    product_name: String,
    original_price: f64,
    tax_rate: f64,
    tax_amount: f64,
    final_price: f64,
}

const FOOD_TAX_RATE: f64 = 0.05;
const ELECTRONICS_TAX_RATE: f64 = 0.10;
const BOOKS_TAX_RATE: f64 = 0.0;

fn calculate_tax(product: &Product) -> TaxCalculation {
    let tax_rate = match product.product_type {
        ProductType::Food => FOOD_TAX_RATE,
        ProductType::Electronics => ELECTRONICS_TAX_RATE,
        ProductType::Books => BOOKS_TAX_RATE,
    };

    let tax_amount = product.price * tax_rate;

    TaxCalculation {
        product_name: product.name.clone(),
        original_price: product.price,
        tax_rate,
        tax_amount,
        final_price: product.price + tax_amount,
    }
}

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

    let products = [book1, smartphone1, food1];

    let mut results = Vec::new();
    for product in &products {
        let tax_result = calculate_tax(product);
        results.push(tax_result);
    }

    for result in &results {
        println!("---");
        println!("Product: {}", result.product_name);
        println!("Original Price: ${:.2}", result.original_price);
        println!("Tax Rate: {:.2}%", result.tax_rate * 100.0);
        println!("Tax Amount: ${:.2}", result.tax_amount);
        println!("Final Price: ${:.2}", result.final_price);
        println!("---");
    }
}
