use rand::Rng;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..101)
}
