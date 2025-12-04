pub enum ProductType {
    Food,
    Electronics,
    Books,
}

pub struct Product {
    pub name: String,
    pub price: f64,
    pub product_type: ProductType,
}