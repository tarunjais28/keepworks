use crate::schema::*;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub email: String,
    #[serde(rename = "line_items")]
    pub line_items: Vec<InputLineItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Insertable, Queryable)]
#[serde(rename_all = "camelCase")]
#[table_name = "orders"]
pub struct InsertOrder {
    pub email: String,
}

#[derive(Debug, Copy, Serialize, Deserialize, Eq, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputLineItem {
    pub quantity: i32,
    #[serde(rename = "product_id")]
    pub product_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder {
    pub order: Order,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "line_items"]
pub struct InsertLineItem {
    pub order_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "products"]
pub struct InsertProduct {
    pub product_id: i32,
    pub item_id: i32,
    pub product_name: String,
}

impl PlaceOrder {
    pub fn place_order(self, connection: &PgConnection) -> Result<Self, Error> {
        use crate::schema::orders::dsl::*;
        use diesel::dsl::max;

        let insert_order = InsertOrder {
            email: self.order.email.to_string(),
        };

        // Inserting records to customer table
        diesel::insert_into(orders)
            .values(&insert_order)
            .execute(connection)
            .expect("Error while inserting data to `orders` table.");

        // Taking latest order_id from Customer table to insert it into Product table as foreign key
        let recent_order_id: Option<i32> = orders
            .select(max(order_id))
            .first(connection)
            .expect("Error while getting recent `order_id` from `orders` table.");
        let item_order_id =
            recent_order_id.expect("Error while getting `order_id` from `orders` table.");

        let mut items: Vec<InputLineItem> = Vec::new();
        // Iterating each item from the pool of line_items
        for each_item in &self.order.line_items {
            let item = InsertLineItem {
                order_id: item_order_id,
                quantity: each_item.quantity,
            };
            items.push(*each_item);

            // Inserting product details to line_items table
            use crate::schema::line_items::dsl::*;
            diesel::insert_into(line_items)
                .values(&item)
                .execute(connection)
                .expect("Error while inserting data to `products` table.");

            // Inserting product_details to product table from product_stock table
            // Extracting recent item id from line_items table
            let recent_item_id: Option<i32> = line_items
                .select(max(crate::schema::line_items::item_id))
                .first(connection)
                .expect("Error while getting recent `item_id` from `line_items` table.");
            let prod_item_id =
                recent_item_id.expect("Error while getting `item_id` from `line_items` table.");

            // Extracting product info from the product_stock table
            use crate::schema::product_stock::dsl::*;
            let prod_name: String = product_stock
                .filter(stock_prod_id.eq(each_item.product_id))
                .select(crate::schema::product_stock::product_name)
                .first(connection)
                .unwrap_or_else(|_| panic!(
                    "Error while getting `product_name` from `product_stock` table for product_id : `{}`.", 
                    each_item.product_id
            ));

            let prod = InsertProduct {
                product_id: each_item.product_id,
                item_id: prod_item_id,
                product_name: prod_name,
            };

            use crate::schema::products::dsl::*;
            diesel::insert_into(products)
                .values(&prod)
                .execute(connection)
                .expect("Error while inserting data to `products` table.");
        }

        Ok(PlaceOrder {
            order: Order {
                email: self.order.email,
                line_items: items,
            },
        })
    }
}
