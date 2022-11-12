// Cool mais comme on est des grosses feignasses d'informaticien et que moins on en tape mieux
// c'est...
// On peut avoir une implémentation automatique du trait Debug avec une macro #[derive...]
//
// Donc on vire l'implémentation manuelle et on ajoute la macro.
//                                                         |
//  --------------------------------------------------------
//  |
//  v
#[derive(Debug)]
enum Planets {
    Tatooine,
    Alderaan,
    Coruscant,
    Dagobah,
    Mustafar,
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

// Bing ça marche comme avant !!!
//
//  🦉 uggla   master  …  rfs  rfs_07  solution_03  cargo run
//    Compiling solution_03 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_07/solution_03)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.43s
//      Running `target/debug/solution_03`
// Flight to Tatooine !
// Flight to Alderaan !
// Flight to Coruscant !
// Flight to Dagobah !
// Flight to Mustafar !
// A very long trip !
//
//

// Et si on fait une expansion des macros
//
//
//  🦉 uggla   master  …  rfs  rfs_07  solution_03  cargo expand
//     Checking solution_03 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_07/solution_03)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.89s
//
// #![feature(prelude_import)]
// #[prelude_import]
// use std::prelude::rust_2021::*;
// #[macro_use]
// extern crate std;
// enum Planets {
//     Tatooine,
//     Alderaan,
//     Coruscant,
//     Dagobah,
//     Mustafar,
// }
//

//     -- La macro #[derive....], (comment on écrit des macros comme ça c'est pour plus tard).
//     |  On voit que l'implémentation est quasi la même que l'on avait fait manuellement.
//     v
// #[automatically_derived]
// impl ::core::fmt::Debug for Planets {
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         match self {
//             Planets::Tatooine => ::core::fmt::Formatter::write_str(f, "Tatooine"),
//             Planets::Alderaan => ::core::fmt::Formatter::write_str(f, "Alderaan"),
//             Planets::Coruscant => ::core::fmt::Formatter::write_str(f, "Coruscant"),
//             Planets::Dagobah => ::core::fmt::Formatter::write_str(f, "Dagobah"),
//             Planets::Mustafar => ::core::fmt::Formatter::write_str(f, "Mustafar"),
//         }
//     }
// }
// fn main() {
//     let destinations = [
//         Planets::Tatooine,
//         Planets::Alderaan,
//         Planets::Coruscant,
//         Planets::Dagobah,
//         Planets::Mustafar,
//     ];
//     for dest in destinations {
//         {
//             ::std::io::_print(::core::fmt::Arguments::new_v1(
//                 &["Flight to ", " !\n"],
//                 &[::core::fmt::ArgumentV1::new_debug(&dest)],
//             ));
//         };
//     }
//     {
//         ::std::io::_print(::core::fmt::Arguments::new_v1(
//             &["A very long trip !\n"],
//             &[],
//         ));
//     };
// }
