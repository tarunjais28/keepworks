use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
    #[serde(rename = "orders")]
    pub order: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "id")]
    pub order_id: i32,
    pub email: String,
    #[serde(rename = "line_items")]
    pub line_items: Vec<LineItems>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineItems {
    pub quantity: i32,
    pub product: Products,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    #[serde(rename = "id")]
    pub product_id: i32,
    #[serde(rename = "name")]
    pub product_name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct OrderInfo {
    pub order_id: i32,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Copy, Clone)]
pub struct LineItemInfo {
    pub item_id: i32,
    pub quantity: i32,
}

impl Orders {
    pub fn get_orders(connection: &PgConnection) -> Result<Self, Error> {
        use crate::schema::line_items::dsl::*;
        use crate::schema::orders::dsl::*;
        use crate::schema::products::dsl::*;

        let order_info: Vec<OrderInfo> = orders
            .select((
                crate::schema::orders::order_id,
                crate::schema::orders::email,
            ))
            .order_by(crate::schema::orders::created_at.asc())
            .load(connection)
            .expect("Error while loading data from `orders` table.");

        let mut order: Vec<Order> = Vec::new();
        let mut line_item: Vec<LineItems> = Vec::new();
        for each_order in order_info.iter() {
            let item_info: Vec<LineItemInfo> = line_items
                .filter(crate::schema::line_items::order_id.eq(each_order.order_id))
                .select((
                    crate::schema::line_items::item_id,
                    crate::schema::line_items::quantity,
                ))
                .order_by(crate::schema::line_items::created_at.asc())
                .load(connection)
                .unwrap_or_else(|_| {
                    panic!(
                        "Error while loading data from `line_item` table for order_id: `{}`.",
                        each_order.order_id
                    )
                });

            for each_item in item_info.iter() {
                let mut product_info: Vec<Products> = products
                .filter(crate::schema::products::item_id.eq(each_item.item_id))
                .select((crate::schema::products::product_id,crate::schema::products::product_name))
                .order_by(crate::schema::products::created_at.asc())
                .load(connection)
                .unwrap_or_else(|_| panic!(
                        "Error while loading data from `products` table for order_id: `{}` and item_id: `{}`.", 
                            each_order.order_id, each_item.item_id));

                // As item_id and product_id is primary key, there will always be only one product corresponding
                // to given item_id
                let prod_info: Products = product_info.pop().
                unwrap_or_else(|| 
                    panic!(
                        "Error while loading product from the pool of products for order_id: `{}` and item_id: `{}`.",
                         each_order.order_id, each_item.item_id));
                let item = LineItems {
                    quantity: each_item.quantity,
                    product: prod_info,
                };
                line_item.push(item);
            }
            order.push(Order {
                order_id: each_order.order_id,
                email: each_order.email.to_string(),
                line_items: line_item.clone(),
            });
            line_item.clear();
        }

        Ok(Orders { order })
    }
}
