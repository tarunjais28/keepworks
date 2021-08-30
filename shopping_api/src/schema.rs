table! {
    line_items (item_id) {
        item_id -> Int4,
        order_id -> Int4,
        quantity -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    orders (order_id) {
        order_id -> Int4,
        email -> Text,
        created_at -> Timestamp,
    }
}

table! {
    products (product_id, item_id) {
        product_id -> Int4,
        item_id -> Int4,
        product_name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    product_stock (stock_prod_id) {
        stock_prod_id -> Int4,
        product_name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_name -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
    }
}

joinable!(line_items -> orders (order_id));
joinable!(products -> line_items (item_id));

allow_tables_to_appear_in_same_query!(line_items, orders, products, product_stock, users,);
