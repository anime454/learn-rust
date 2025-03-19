
fn default_immutable() {
    // In a initialization, the variable is immutable by default
    let x = 5;
    println!("The value of x is: {x}");

    // Since x is mutable we can change its value
    x = 6;
    println!("The value of x is: {x}");
}

fn fixed_mutable() {
    // We can also declare a variable as mutable
    let mut y = 5;
    println!("The value of y is: {y}");

    // Since y is mutable we can change its value
    y = 6;
    println!("The value of y is: {y}");
}

fn shadowing() {
    // We can also shadow a variable
    let z = 5;
    println!("The value of z is: {z}");

    // We can shadow z with a new variable
    let z = z + 1;
    println!("The value of z is: {z}");

    // We can shadow z with a new variable of a different type
    let z = "Hello";
    println!("The value of z is: {z}");
}


fn main() {
    default_immutable();
    fixed_mutable();
    shadowing();
}