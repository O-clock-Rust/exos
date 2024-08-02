// Définition de la struct Car
struct Car {
    make: String,
    model: String,
    year: u32,
}

// Implémentation de méthodes pour Car
impl Car {
    // Méthode pour calculer l'âge de la voiture
    fn car_age(&self, current_year: u32) -> u32 {
        current_year - self.year
    }

    // Fonction pour vérifier si la voiture est ancienne
    fn is_classic(&self, current_year: u32) -> Option<&Car> {
        if self.car_age(current_year) > 10 {
            Some(self)
        } else {
            None
        }
    }

    // Fonction pour démarrer la voiture
    fn start_car(&self, battery_ok: bool) -> Result<&Car, String> {
        if battery_ok {
            Ok(self)
        } else {
            Err(String::from("Cannot start the car: battery is dead"))
        }
    }
}

fn main() {
    let my_car = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2010,
    };

    // Calcul de l'âge de la voiture
    let current_year = 2024;
    println!("Car age: {} years", my_car.car_age(current_year));

    // Vérifier si la voiture est ancienne
    match my_car.is_classic(current_year) {
        Some(car) => println!("The car is a classic: {}", car.model),
        None => println!("The car is not a classic"),
    }

    // Démarrer la voiture avec batterie OK
    match my_car.start_car(true) {
        Ok(car) => println!("Car started successfully: {}", car.model),
        Err(e) => println!("Error: {}", e),
    }

    // Démarrer la voiture avec batterie déchargée
    match my_car.start_car(false) {
        Ok(car) => println!("Car started successfully: {}", car.model),
        Err(e) => println!("Error: {}", e),
    }
}
