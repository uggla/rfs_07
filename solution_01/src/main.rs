enum Planets {
    Tatooine,
    Alderaan,
    Coruscant,
    Dagobah,
    Mustafar,
}

//               v-- On implémente le trait Debug pour notre enum (Planet)
impl std::fmt::Debug for Planets {
    //       On récupère la signature de la fonction depuis la doc que l'on copie ici.
    //       v-- notre type,           v-- en gros un type qui représente le {:?}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        //                                             ^-- la fonction renvoie un Result vide ou
        //                                             avec un fmt::Error (voir la doc):
        //                                             https://doc.rust-lang.org/std/fmt/trait.Debug.html#required-methods
        //                                             si on clique sur Error https://doc.rust-lang.org/std/fmt/struct.Error.html
        //                                             A noter que pour faire moins long à écrite on peut
        //                                             remplacer par un std::fmt::Result = Result<(), std::fmt::Error>

        // v-- on renvoie la chaine de manière inconditionnelle.
        f.write_str("which planet !!!")
    }
}

fn main() {
    let p1 = Planets::Dagobah;
    println!("Flight to {:?}", p1);
    //                    ^- On se fait plus crier dessus le compilo est content a part qq warning.
    //                    Mais ca compile et fait ce que l'on a demandé cad afficher
    //                    "which planet !!! ".
}

//  🦉 uggla   master  …  rfs  rfs_07  solution_01  cargo run
//    Compiling solution_01 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_07/solution_01)
// warning: variants `Tatooine`, `Alderaan`, `Coruscant` and `Mustafar` are never constructe
// d
//  --> src/main.rs:2:5
//   |
// 1 | enum Planets {
//   |      ------- variants in this enum
// 2 |     Tatooine,
//   |     ^^^^^^^^
// 3 |     Alderaan,
//   |     ^^^^^^^^
// 4 |     Coruscant,
//   |     ^^^^^^^^^
// 5 |     Dagobah,
// 6 |     Mustafar,
//   |     ^^^^^^^^
//   |
//   = note: `#[warn(dead_code)]` on by default
//
// warning: `solution_01` (bin "solution_01") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 0.67s
//      Running `target/debug/solution_01`
// Flight to Which planet !!!
//  🦉 uggla   master  …  rfs  rfs_07  solution_01  cargo run
