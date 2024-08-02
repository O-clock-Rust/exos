1. Définir un trait `Shape` qui comporte une méthode `area`. La méthode doit retourner un `f64` qui représente la superficie de la forme


2. Créer deux structs :

    a. `Rectangle` : avec les champs `width` et `height`, tous deux de type `f64`

    b. `Circle` : avec le champ `radius` de type `f64`


3. Implémenter le trait `Shape` sur les structs

    a. Pour `Rectangle`, la méthode `area` doit calculer la superficie du rectangle (largeur x hauteur)

    b. Pour `Circle`, la méthode `area` doit calculer la superficie du cercle (`π×radius2`)

    _on peut faire appel à PI comme ceci en Rust : `std::f64::consts::PI`_

4. Ecrire une fonction `print_area` qui prend en paramètre une référence à `Shape`

La fonction doit appeler la méthode area sur l’objet `Shape` et afficher le résultat.


5. Tester le code dans `main`. Créer une instance des deux structs, et appeler la fonction `print_area` avec les deux instances pour confirmer que les superficies sont calculées et affichées correctement.
