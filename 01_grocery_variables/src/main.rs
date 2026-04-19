fn main() {
    // TODO 1: Declare an uninitialized variable named `apple_quantity` with the explicit type `i32`.
    let apple_quantity: i32;
    

    // TODO 2: Declare an uninitialized variable named `apple_price` with the explicit type `f64`.
    let apple_price: f64;

    // TODO 3: Declare a variable named `apple_review` with the explicit type `f64` and assign it the value `0.52`.
    let apple_review: f64 = 0.52;

    // TODO 4: Declare an uninitialized variable named `apple_review_display` with the explicit type `i32`.
    let apple_review_display: i32;

    // TODO 5: Declare a constant named `APPLE_LOCATION` with the explicit type `char` and assign it the value `'F'`.
    const APPLE_LOCATION: char = 'F';

    // TODO 6: Assign the value `1.49` to the previously declared `apple_price` variable.
    apple_price = 1.49;

    // TODO 7: Assign the value `23` to the previously declared `apple_quantity` variable.
    apple_quantity = 23;

    // TODO 8: Multiply `apple_review` by `100.0`, explicitly cast the result to `i32` using the `as` keyword, and assign it to `apple_review_display`.
    apple_review_display = (apple_review * 100.0) as i32;

    // TODO 9: Use the `println!` macro to display the final message. 
    // Format: "An apple cost: {}, there are {} in inventory found in section: {} and your customer gave it an average review of {}"
    println!("An apple const: {}, there are {} in inventory found in section: {} and your customer gave it an  average review of {}%",
             apple_price, apple_quantity, APPLE_LOCATION, apple_review_display);
}
