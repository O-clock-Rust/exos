// 1. Définir le trait `Shape` avec une méthode `area` qui retourne la superficie.
trait Shape {
    fn area(&self) -> f64;
}

// 2. Créer une structure `Rectangle` et implémenter le trait `Shape` pour elle.
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 3. Créer une structure `Circle` et implémenter le trait `Shape` pour elle.
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 4. Écrire une fonction `print_area` qui prend une référence à un `Shape` et affiche la superficie.
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle { radius: 7.0 };

    // Utiliser la fonction `print_area` pour afficher la superficie des formes
    print_area(&rectangle); // Affiche: Area: 50
    print_area(&circle); // Affiche: Area: 153.93804002589985
}
