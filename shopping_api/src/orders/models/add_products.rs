use crate::schema::*;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "product_stock"]
pub struct Product {
    pub product_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProductStock {
    pub products: Vec<Product>,
}

impl ProductStock {
    pub fn insert_products(self, connection: &PgConnection) -> Result<Self, Error> {
        use crate::schema::product_stock::dsl::*;

        // Inserting records to customer table
        for product in self.products.iter() {
            diesel::insert_into(product_stock)
                .values(product)
                .execute(connection)
                .expect("Error while inserting data to `product_stock` table.");
        }
        Ok(ProductStock {
            products: self.products,
        })
    }
}
