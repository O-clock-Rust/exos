1. Reprendre l’exercice précédent


2. Créer une fonction `takes_reference` qui prend pour propriété une variable de type `&String`

Dans cette fonction, comme pour `takes_ownership` on ne cherche qu’à imprimer le texte dans la console


3. Dans la fonction `main`, remplacer l’appel à `takes_ownership` par un appel à `takes_reference`

Cette fois-ci, la propriété passée dans `takes_reference` doit être une référence (`&`)


4. Ne pas toucher au reste du code de la fonction `main`


5. Lancer le programme, cette fois-ci il ne devrait pas y avoir d’erreur, malgré la présence de deux impressions (une dans `main` et une dans `takes_reference`
