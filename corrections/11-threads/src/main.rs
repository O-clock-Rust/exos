use std::thread;

fn main() {
    let exponents = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut handles = vec![];

    for &exp in &exponents {
        let handle = thread::spawn(move || {
            let result = 2_i32.pow(exp);
            println!("2^{} = {}", exp, result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
