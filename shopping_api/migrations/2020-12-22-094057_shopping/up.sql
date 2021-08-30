CREATE TABLE orders (
  order_id SERIAL NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (order_id)
);

CREATE TABLE line_items (
  item_id SERIAL,
  order_id INT NOT NULL,
  quantity INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (item_id),
  FOREIGN KEY (order_id) REFERENCES orders(order_id)
);

CREATE TABLE products (
  product_id INT,
  item_id INT NOT NULL,
  product_name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (product_id,item_id),
  FOREIGN KEY (item_id) REFERENCES line_items(item_id)
);

CREATE TABLE product_stock (
  stock_prod_id SERIAL,
  product_name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (stock_prod_id)
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_name TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now()
);