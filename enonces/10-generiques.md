**Ecrire un programme qui permet d'afficher un contenu dans la console peu importe son type**

1. Créer un struct qui comporte un générique `T` et un membre `value: T`
2. Implémenter les méthodes `new` et `debug_info` qui permettent respectivement de créer un nouveau struct, et d'effectuer un `println!` de la valeur.

➡️ Pour inclure Debug à notre générique, on le spécifie ainsi : `impl<T: Debug> for`

3. Utiliser la fonction `main` pour tester le fonctionnement des méthodes. Par exemple, essayer au moins d'afficher un nombre et une chaîne de caractères à l'aide de la méthode `debug_info`