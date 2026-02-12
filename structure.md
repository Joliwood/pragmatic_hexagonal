### Use cases

1 - Flower Delivery
- The user wants to schedule a flower delivery from an order number (step 5)
- The billing service wants to confirm

2 - Inventory Management
- Get the number of available flowers (steps 1 & 2)
- Reserve flowers and/or tools (step 7)

3 - Billing
- Not detailed for the example

### Step by step

0 > [Client] -> [Inventory] > The user wants to know the stock for a specific flower type / tool

1 > [Inventory] -> [Client] > Is the stock sufficient?

2 + 3 > [Client] -> [Billing] > I want to create a cart from flowers and/or tools

4 > [Billing] -> [Client] > The cart is created, here is the order number + price

5 > [Client] -> [Delivery] > The user wants to schedule a flower delivery from an order number (choosing the delivery type and providing their address)

6 > [Delivery] -> [Billing] > Update the cart price (order ref) based on the delivery type

6.5 > [Client] -> [Billing] > The user accepts + pays the total price

7 > [Billing] -> [Delivery] > Confirm the delivery, the client has paid (with order ref)

7.5 > [Billing] -> [Inventory] > Reserve the flowers and/or tools
