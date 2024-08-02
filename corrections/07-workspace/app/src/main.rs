fn main() {
    let a = 10;
    let b = 5;
    println!("{} + {} = {}", a, b, math_operations::add(a, b));

    let random = math_operations::random_number();
    println!("Generated random number: {}", random);
}
