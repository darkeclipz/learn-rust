mod entity;
mod attribute;

use std::i32;

use crate::entity::{Entity, Weapon, Die};

fn main() {
    println!("Hello, world!");
    create_entity_test();

    let mut some_struct: SomeStruct = SomeStruct { num: 3 };

    print_some_struct(&some_struct);
    mutate_struct(&mut some_struct);
    print_some_struct(&some_struct);

    biggest(&some_struct, &some_struct);
    
    rpg_test();
}

fn create_entity_test() {

    let mut e1: Entity = Entity::new(String::from("Entity1"));
    
    e1.add_damage(10);
    
    println!("{:?}", e1);

    let mut i = 0;
    while i < 5 {
        println!("{i}");
        i += 1;
    }
}

#[derive(Debug)]
struct SomeStruct {
    num: i32
}

fn print_some_struct(the_struct: &SomeStruct) {
    println!("{:?}", the_struct);
}

fn mutate_struct(the_struct: &mut SomeStruct) {
    the_struct.num = 5;
}

fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &SomeStruct {
    if a.num > b.num {
        a
    }
    else {
        b
    }
}

fn rpg_test() {

    let mut player: Entity = Entity::new(String::from("Player"));
    let mut enemy: Entity = Entity::new(String::from("Enemy"));
    
    let d8 = Die { eyes: 8 };
    let w1 = Weapon { name: String::from("Homunculus greatsword"), die: d8.clone(), value: 1_000_000 };
    let w2: Weapon = Weapon { name: String::from("Orc club"), die: d8.clone(), value: 100 };

    player.equip_weapon(w1);  
    enemy.equip_weapon(w2);
    
    let mut player1 : Entity = player;
    let mut player2 : Entity = enemy;
    let mut max_turns = 100;

    while player1.health > 0 && player2.health > 0 {

        let is_hit = player1.roll_hit(&player2);

        if is_hit {
            let dmg = player1.roll_dmg();
            player2.add_damage(dmg);
            println!("{} received {}🪓 damage.", player2.name, dmg);
        }
        else {
            println!("{} missed!", player1.name);
        }

        (player1, player2) = (player2, player1);
        max_turns -= 1;

        if max_turns < 0 {
            panic!("Max turns reached, aborting!");
        }
    }

    print!("{} has won!", if player1.health > player2.health { player1.name } else { player2.name });

}
