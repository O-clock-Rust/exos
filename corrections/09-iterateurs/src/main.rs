fn main() {
    // Liste de nombres à tester
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // Fonction pour vérifier si un nombre est premier
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    // Filtrer les nombres premiers et collecter les résultats
    let prime_numbers: Vec<u32> = numbers
        .into_iter()
        .filter(|&x| is_prime(x)) // Utilisation de la fonction is_prime pour filtrer
        .collect(); // Collecter les nombres premiers dans un vecteur

    // Affichage des nombres premiers
    println!("Les nombres premiers sont : {:?}", prime_numbers);
}
