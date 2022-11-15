// Bon on peut améliorer notre implémentation pour réagir en fonction des planètes.

enum Planet {
    Tatooine,
    Alderaan,
    Coruscant,
    Dagobah,
    Mustafar,
}

impl std::fmt::Debug for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // v--- on fait un pattern matching sur notre enum et on définit la sortie en fonction de
        //      la planète.
        match self {
            Planet::Tatooine => f.write_str("Dagobah"),
            Planet::Alderaan => f.write_str("Alderaan"),
            Planet::Coruscant => f.write_str("Coruscant"),
            Planet::Dagobah => f.write_str("Dagobah"),
            Planet::Mustafar => f.write_str("Mustafer"),
        }
    }
}

fn main() {
    let destinations = [
        Planet::Tatooine,
        Planet::Alderaan,
        Planet::Coruscant,
        Planet::Dagobah,
        Planet::Mustafar,
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
