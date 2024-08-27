1. Définir un struct `Book` selon les instructions suivantes :
    - Champs : `title`, `author`, `year` et `is_borrowed`
    - Méthode : `book_age` permettant de calculer l’âge du livre (en lui donnant une année en argument)
    - Créer la méthode `describe` qui formate une chaîne de caractères
    pour décrire l'instance du struct
    (ex : « The book "The Hitchhiker's Guide to the Galaxy" by Douglas Adams
    was published in 1979. »)
    - [Optionnel] Créer le contructeur (Méthode associée `new`) ; `is_borrowed` est `false` par défaut

2. Écrire un programme permettant de créer une instance du struct `Book`
et affichant la description du livre dans la console

3. Définir une fonction `is_classic` qui retourne une `Option`
pour vérifier si le livre est ancien (+ de 50 ans)…
peut-être utiliser `book_age` dans la fonction ;)

4. Définir une fonction `borrow` qui retourne un `Result`
pour emprunter le livre, avec une erreur si le livre est déjà emprunté

    > pensez à modifier l'instance dans la méthode
    > pour passer `self.is_borrowed` à `true`  
    > (si le livre peut être emprunté)

5. Compléter le programme en incorporant des appels aux fonctions `is_classic` et `borrow`
