# Shoping API

# Initialize POSTGRES Database

```sh
Check .env file for Host IP and Post Number

run below command to setup diesel
diesel setup

```
## Available routes
# POST /register
# POST /add_products/{user_name}/{password}
# GET /products/{user_name}/{password}
# POST /place_order/{user_name}/{password}
# GET /orders/{user_name}/{password}
# GET /show_users/{user_name}/{password}


# For registering user, use below command

This service is used to register new user for authentication.

```sh
curl -S -X POST --header "Content-Type: application/json" --data '{ "user_name": "ecom", "password": "ecom@123" }' http://localhost:8080/register

```

# For POST products, use below command

This service is used to add products in stock. 

```sh
curl -S -X POST --header "Content-Type: application/json" --data '{ "products": [ { "product_name": "Product 1" }, { "product_name": "Product 2" }, { "product_name": "Product 3" }, { "product_name": "Product 4" }, { "product_name": "Product 5" } ] }' http://localhost:8080/add_products/{user}/{pass}

```
## Note: This service is required to be called, so that the orders be placed without error


# For GET products, use below command

This service lists the products in stock. 

```sh
curl -S http://localhost:8080/products/{user}/{pass}

```

# For POST order, use below command

This service places the new orders. 

```sh
curl -S -X POST --header "Content-Type: application/json" --data '{ "order": { "email": "test@example.com", "line_items": [ { "quantity": 5, "product_id": 1 }, { "quantity": 2, "product_id": 2 } ] } }' http://localhost:8080/place_order/{user}/{pass}

```

# For GET order, use below command

This service shows the complete order details for various placed orders.

```sh
curl -S http://localhost:8080/orders/{user}/{pass}

```

# For GET list of users, use below command

This service shows the list of users, authenticated to use the services.

```sh
curl -S http://localhost:8080/show_users/{user}/{pass}

```

# Terminology
`{user}` : username to be provided while executing services
`{pass}` : password to be provided while executing services
