// Bon on peut améliorer notre implémentation pour réagir en fonction des planètes.

enum Planets {
    Tatooine,
    Alderaan,
    Coruscant,
    Dagobah,
    Mustafar,
}

impl std::fmt::Debug for Planets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // v--- on fait un pattern matching sur notre enum et on définit la sortie en fonction de
        //      la planète.
        match self {
            Planets::Tatooine => f.write_str("Dagobah"),
            Planets::Alderaan => f.write_str("Alderaan"),
            Planets::Coruscant => f.write_str("Coruscant"),
            Planets::Dagobah => f.write_str("Dagobah"),
            Planets::Mustafar => f.write_str("Mustafer"),
        }
    }
}

fn main() {
    let destinations = [
        Planets::Tatooine,
        Planets::Alderaan,
        Planets::Coruscant,
        Planets::Dagobah,
        Planets::Mustafar,
    ];

    for dest in destinations {
        println!("Flight to {:?} !", dest);
    }

    println!("A very long trip !");
}

//  🦉 uggla   master  …  rfs  rfs_07  solution_02  cargo run
//    Compiling solution_02 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_07/solution_02)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.33s
//      Running `target/debug/solution_02`
// Flight to Dagobah !
// Flight to Alderaan !
// Flight to Coruscant !
// Flight to Dagobah !
// Flight to Mustafer !
// A very long trip !
