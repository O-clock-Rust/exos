// Définition de la struct Car
struct Car {
    brand: String,
    model: String,
    year: u32,
}

// Implémentation de méthodes pour Car
impl Car {
    // Constructeur [optionnel]
    fn new(brand: String, model: String, year: u32) -> Self {
        Car { brand, model, year }
    }

    // Méthode pour décrire l'instance [optionnel]
    fn describe(&self) -> String {
        format!("In {}, {} created the {}", self.year, self.brand, self.model)
    }

    /* DEPRECATED : ne gère pas quand current_year < self.year
    // Méthode pour calculer l'âge de la voiture
    fn car_age(&self, current_year: u32) -> u32 {
        current_year - self.year
    }

    // Fonction pour vérifier si la voiture est ancienne
    fn is_classic(&self, current_year: u32) -> Option<&Self> {
        if self.car_age(current_year) > 10 {
            Some(self)
        } else {
            None
        }
    }
    */

    // Méthode pour calculer l'âge de la voiture
    fn car_age(&self, current_year: u32) -> Result<u32, String> {
        if current_year < self.year {
            Err(String::from("Current year cannot be less than the car's year of manufacture."))
        } else {
            Ok(current_year - self.year)
        }
    }

    // Fonction pour vérifier si la voiture est ancienne
    fn is_classic(&self, current_year: u32) -> Result<bool, String> {
        let age = self.car_age(current_year)?;
        Ok(age > 10)
    }
}

fn main() {
    let my_car = Car::new(
        String::from("Fiat"),
        String::from("Multipla"),
        1998
    );
    println!("{}", my_car.describe());
    // println!("Age of my car: {}", my_car.car_age(2024));

    /* DEPRECATED
    // Vérifier si la voiture est ancienne
    match my_car.is_classic(2024) {
        Some(_car) => println!("This car is a classic!"),
        None => println!("This car is not a classic.")
    }
    */

    // Vérifier si la voiture est ancienne
    match my_car.is_classic(2024) {
        Ok(true) => println!("This car is a classic!"),
        Ok(false) => println!("This car is not a classic."),
        Err(e) => println!("Error: {}", e)
    }
}
