// Struct Book
struct Book {
    title: String,
    author: String,
    year: u32,
    is_borrowed: bool,
}

impl Book {
    // Constructeur pour Book
    fn new(title: String, author: String, year: u32) -> Self {
        Book { 
            title, 
            author, 
            year, 
            is_borrowed: false 
        }
    }
    
    // Méthode pour décrire l'instance
    fn describe(&self) -> String {
        format!("The book `{}` by {} was published in {}.",
            self.title,
            self.author,
            self.year
        )
    }

    // Méthode pour calculer l'âge du livre
    fn book_age(&self, current_year: u32) -> u32 {
        current_year - self.year
    }

    // Méthode pour vérifier si le livre est un classique
    fn is_classic(&self, current_year: u32) -> Option<&Self> {
        if self.book_age(current_year) > 50 {
            Some(self)
        } else {
            None
        }
    }

    // Méthode pour emprunter le livre
    fn borrow(&mut self) -> Result<&Self, String> {
        if self.is_borrowed {
            Err(String::from("The book is already borrowed."))
        } else {
            // le livre est en train d'être emprunté, je modifie son statut
            self.is_borrowed = true;
            Ok(self)
        }
    }
}

fn main() {
    let current_year = 2024;

    // Créer une instance de Book
    let mut my_book = Book::new(
        String::from("The Hitchhiker's Guide to the Galaxy"),
        String::from("Douglas Adams"),
        1979
    );

    // Afficher l'âge du livre
    println!("{}", my_book.describe());

    // Vérifier si le livre est un classique
    match my_book.is_classic(current_year) {
        Some(_book) => println!("This book is a classic!"),
        None => println!("This book is not a classic."),
    }

    // Essayer d'emprunter le livre
    match my_book.borrow() {
        Ok(book) => println!("You have borrowed `{}`.", book.title),
        Err(e) => println!("Error: {}", e),
    }

    // Essayer de réemprunter le livre
    match my_book.borrow() {
        Ok(book) => println!("You have borrowed `{}`.", book.title),
        Err(e) => println!("Error: {}", e),
    }
}
