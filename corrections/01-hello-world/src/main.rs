fn main() {
    println!("Hello, Rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_work() {
        assert_eq!(2 * 2, 4);
    }
}
