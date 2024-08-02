// 1. Définir la macro `create_greeting` pour générer une fonction de salutation.
macro_rules! create_greeting {
    ($name:ident) => {
        // Générer une fonction avec le nom fourni
        fn $name() {
            println!("Hello, {}!", stringify!($name));
        }
    };
}

// 2. Utiliser la macro pour générer des fonctions de salutation pour différents noms.
create_greeting!(alice);
create_greeting!(bob);
create_greeting!(charlie);

fn main() {
    // 3. Tester les fonctions générées
    alice(); // Affiche: Hello, Alice!
    bob(); // Affiche: Hello, Bob!
    charlie(); // Affiche: Hello, Charlie!
}
