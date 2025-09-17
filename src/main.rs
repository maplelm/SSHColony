#![allow(dead_code, unused)]
mod engine;
mod game;
mod log;

use engine::{Instance, render::Canvas};
use game::{
    entity::Entity,
    material::{self, Material},
};
use ron;
use std::collections::HashMap;
use std::process::ExitCode;

use crate::{
    engine::types::{Position, Position3D, Store},
    game::{
        Creatures,
        entity::{Common, Kind},
    },
};

fn main() -> ExitCode {
    let ms = vec![
        Material::default(),
        Material::default(),
        Material::default(),
    ];

    let es = vec![
        Entity {
            common: Common {
                name: "dwarf".to_string(),
                flags: 0,
                position: Position3D { x: 0, y: 0, z: 0 },
                visual_position: None,
                inventory: vec![],
                max_inventory_weight: 10,
                stats: Store::default(),
            },
            kind: Kind::Creature {
                kind: Creatures::Dwarf,
                state: game::CreatureState::Idle,
            },
        },
        Entity {
            common: Common {
                name: "Human".to_string(),
                flags: 0,
                position: Position3D { x: 0, y: 0, z: 0 },
                visual_position: None,
                inventory: vec![],
                max_inventory_weight: 10,
                stats: Store::default(),
            },
            kind: Kind::Creature {
                kind: Creatures::Dwarf,
                state: game::CreatureState::Idle,
            },
        }
    ];
    /*
    println!(
        "{}\n\n\n##########\n\n",
        ron::ser::to_string_pretty(&ms, ron::ser::PrettyConfig::default()).unwrap()
    );
    println!(
        "{}",
        ron::ser::to_string_pretty(&es, ron::ser::PrettyConfig::default()).unwrap()
    );
*/
    let x = Store::<Entity>::from_file("./data/entities/creatures.dat").unwrap();

    println!("\n\n");
    println!("dwarf:\n{:?}", x.get("dwarf".to_string()).ok().unwrap().unwrap());

    /*
    let _ = engine::run(Instance::new(
        MainMenu::new(),
        Canvas {
            width: 100,
            height: 50,
        },
    ));
    */
    ExitCode::SUCCESS
}
