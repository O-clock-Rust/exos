1. Définir un struct `Car` selon les instructions suivantes :
        a. Champs : `brand`, `model` et `year`
        b. Méthode : `car_age` permettant de calculer l’âge de la voiture (en lui donnant une année en argument)
        c. [Optionnel] Créer le contructeur (Méthode associée `new`)
        d. [Optionnel] Créer la méthode `describe` qui formate une chaîne de caractères pour décrire l'instance du struct (ex : « In 1998, Fiat created the Multipla »)

2. Écrire un programme permettant de créer une instance du struct `Car` et affichant l’âge de la voiture dans la console

3. Définir une fonction `is_classic` qui retourne une `Option` pour vérifier si la voiture est ancienne (+ de 10 ans)… peut-être utiliser `car_age` dans la fonction ;)

4. Compléter le programme en incorporant un appel à la fonction `is_classic` (avec l'année actuelle comme argument)

5. Modifier l'année actuelle (dans l'appel de `is_classic`) par `1900`… Que se passe-t-il ?

        → Le programme panique ! Rust rencontre une situation qu'il ne peut
        pas gérer correctement : (ici) convertir un nombre négatif (`1900 - 1998`) en type `u32` (_unsigned_).  
        En mode _Debug_, le programme se termine avec un message d'erreur.

6. Corriger la méthode `car_age` : elle doit retourner un `Result` :
        - l'âge de la voiture si celui-ci est positif
        - une erreur ("Current year cannot be less than the car's year of manufacture.") sinon

7. Transformer la méthode `is_classic` pour utiliser directement ce Résult et renvoyer un `Result<bool, String>` grâce à l'opérateur `?`

        > **ASTUCE** dans le `Ok` retourné par `is_classic`,
        > vous pouvez transmettre une condition…  
        > Celle-ci sera gérée dans le `match` grâce à :
        >   - `Ok(true)`
        >   - `Ok(false)`
        >   - `Err(e)`
