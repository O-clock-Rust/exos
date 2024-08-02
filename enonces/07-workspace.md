**Créer un workspace en Rust avec plusieurs crates.**


1. Créer un workspace selon l’architecture ci-après :

```sh
my_workspace/
├── Cargo.toml
├── math_operations/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── app/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
```



2. Dans la crate `math_operations`, créer deux fonctions, l’une permettant de retourner la somme de deux entiers, l’autre de retourner un nombre aléatoire

=> Pour générer le nombre aléatoire, utiliser une crate externe (crates.io)


3. Dans la crate `app`, et sa fonction `main`, importer les fonctions de `math_operations` et les appeler avec des valeurs arbitraires. Les résultats doivent apparaître en console.
