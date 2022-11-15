enum Planet {
    Tatooine,
    Alderaan,
    Coruscant,
    Dagobah,
    Mustafar,
}

fn main() {
    let p1 = Planet::Dagobah;
    println!("Flight to {:?}", p1);
    //                    ^- On peut pas imprimer le debug de notre enum car celui-ci n'implémente pas le trait
    //                    Debug. Un trait, je vois ça comme un comportement. Donc si notre type
    //                    implémente le comportement Debug alors il va afficher ce que l'on veut
    //                    quand on va faire un println!("{:?}", ma_variable_du_type)
    //
    //                    Le trait Debug est un trait de la lib standard rust. On ne peut pas le
    //                    redéfinir, mais on peut faire en sorte que notre type implémente ce trait.
    //                    Pour implémenter le trait Debug il suffit de définir la méthode fmt pour
    //                    notre type. https://doc.rust-lang.org/std/fmt/trait.Debug.html#required-methods
}
