use super::{Deserialize, Serialize};
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{pg::PgConnection, prelude::*, result::Error};

#[derive(Deserialize, Insertable, Serialize, Queryable, Debug, Clone)]
#[table_name = "product_stock"]
#[serde(rename_all = "camelCase")]
pub struct DisplayProduct {
    #[serde(rename = "id")]
    pub stock_prod_id: i32,
    #[serde(rename = "name")]
    pub product_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayProducts {
    pub products: Vec<DisplayProduct>,
}

impl DisplayProducts {
    pub fn get_product_details(connection: &PgConnection) -> Result<Self, Error> {
        use crate::schema::product_stock::dsl::*;
        // Displaying Product details from product table
        let prods: Vec<DisplayProduct> = product_stock
            .select((stock_prod_id, product_name, created_at))
            .order_by(created_at.asc())
            .load(connection)
            .expect("Error while loading data from `product_stock` table.");

        Ok(DisplayProducts { products: prods })
    }
}
