fn main() {
    println!("Hello, world!");

    let player: Entity = {
        health: 0,
        attributes: Attributes {
            strength: 10,
            dexterity: 10,
            constition: 10,
            agility: 10
        }
    };
}



pub struct Entity {
    pub health: u32,
    pub attribute: Attributes,
}

pub struct Attributes {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub agility: u32,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { 
            health: 10, 
            attribute: Attributes {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                agility: 10
            } 
        }
    }
}