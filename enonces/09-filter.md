**Écrire un programme qui utilise des itérateurs pour trouver et collecter les nombres positifs dans une liste de nombres.**

1. Définir le vecteur suivant : `let numbers = vec![-5, 3, -1, 7, 0, -10, 4];`

2. Utiliser filter pour conserver uniquement les nombres positifs, puis collect pour rassembler les résultats dans un vecteur

3. Afficher le résultat en console sous la forme :
`println!("Les nombres positifs dans {:?} sont {:?}", numbers, positives);`


Help :

- il faut utiliser un itérateur qui travaille sur les références (pour pouvoir utiliser numbers par la suite)
- il faudra peut-être COPIÉ l’itérateur de références vers un itérateur de valeurs (https://doc.rust-lang.org/std/iter/trait.Iterator.html)
