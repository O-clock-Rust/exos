enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    let x = 2;
    let y = 2;

    let operation = MathOperation::Multiply;

    let result = match operation {
        MathOperation::Add => x + y,
        MathOperation::Subtract => x - y,
        MathOperation::Multiply => x * y,
        MathOperation::Divide => x / y,
    };

    println!("Result : {result}");
}
