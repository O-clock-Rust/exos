fn main() {
    // Étape 1 : Créer un vecteur d'entiers
    let numbers = vec![-5, 3, -1, 7, 0, -10, 4];

    // Étape 2 : Filtrer les nombres positifs
    let positives: Vec<i32> = numbers
        .iter()                      // Crée un itérateur sur le vecteur
        .filter(|&&n| n > 0)        // Filtre pour garder uniquement les nombres positifs
        .copied()                   // Copie l'itérateur de références en itérateur de valeurs
        .collect();                 // Collecte les résultats dans un vecteur

    // Étape 3 : Afficher les nombres positifs
    println!("Nombres positifs dans {:?} : {:?}", numbers, positives);
}
