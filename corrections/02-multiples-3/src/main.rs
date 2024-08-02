fn main() {
    let a = 5;
    let b = 30;

    for num in a..=b {
        if num % 3 == 0 {
            println!("{}", num);
        }
    }
}
