fn main() {
    let mut width: f64 = 7.5;
    let mut height: f64 = 5.0;

    let mut result = calc_area(width, height);

    println!("Premier résultat : {}", result);

    width = 5.0;
    height = 2.5;

    result = calc_area(width, height);

    println!("Second résultat : {result}"); // On peut aussi passer directement la variable dans le spécificateur de format
}

fn calc_area(x: f64, y: f64) -> f64 {
    x * y
}
