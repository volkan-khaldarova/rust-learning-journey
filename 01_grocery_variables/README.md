# Grocery Store: Apple Inventory (Rust Edition)

## Overview
Grocery stores have lots of information to keep track of: inventory, customers, prices, sales, employees, shipping, and more. We are going to start building a small part of that system here.

For now, we are going to focus on one specific product in a grocery store: apples. We will put together basic information about them and display the information at the end. You will be using the skills you learned about variables, mutability, and data types in Rust to do this work. Let’s get started!

## Tasks

### Part 1: Creating the Variables & Assigning Values
First things first, let’s create our variables where we will store all our information during our program runtime.

1. **Price:** At declaration, create a variable named `apple_price` and set the price of apples to `1.49`. Keep in mind that we are storing a currency value; think about whether `f32` or `f64` is more appropriate for floating-point precision.
2. **Location:** We need to know where to find the apples in the store. Create a variable called `apple_location` and set it to `'F'`. Since this is a single character, ensure you are using the correct Rust scalar type.
3. **Quantity:** Below your initial declarations, declare a variable for the number of apples in inventory. Call this `apple_quantity` and set it to `23`. 
4. **Average Review:** Create a variable to store the average user review and set it to `82.5`. Call it `apple_review`. Choose the appropriate floating-point type.

### Part 2: Constants & Immutability
5. **Location Constraint:** We want to make sure customers can always find apples. Ensure that the `apple_location` variable is immutable so it cannot be changed from what was set at declaration. (Hint: Leverage Rust's default variable behavior or consider declaring it as a `const`).

### Part 3: Type Casting
6. **Display Review:** For display purposes, we don’t need the precision that a floating-point number offers. We want an integer. Declare a new variable called `apple_review_display`.
7. **Casting Operation:** Set the value of `apple_review_display` by casting the `apple_review` variable into an integer. Rust does not support implicit casting, so you will need to explicitly cast the value using the `as` keyword.

### Part 4: Output
8. **Print the Data:** Use the `println!` macro to display all the gathered information clearly to the terminal.
9. **Run:** Execute `cargo run` to compile and run your program. Ensure everything is working as expected!