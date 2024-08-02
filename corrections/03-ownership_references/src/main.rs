fn main() {
    let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s); // Que se passe-t-il ici ?

    takes_reference(&s); // Passe une référence de `s` à la fonction
    println!("{}", s); // Cela fonctionne maintenant
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_reference(some_string: &String) {
    println!("{}", some_string);
}
