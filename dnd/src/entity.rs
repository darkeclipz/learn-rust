use rand::Rng;
use std::string::String;

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub health: u32,
    pub attributes: Attributes,
    pub weapon: Weapon,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub die: Die,
    pub value: u32,
}

impl Weapon {
    pub fn get_damage(&self) -> u32 {
        if self.die.eyes == 0 {
            return 0;
        }
        self.die.throw() as u32
    }

    pub fn default() -> Weapon {
        Weapon {
            name: String::from("NULL Weapon"),
            die: Die { eyes: 0 },
            value: 0,
        }
    }
}

impl Entity {
    pub fn max_health(&self) -> u32 {
        10 + (self.attributes.constitution as f64 / 2.0) as u32
    }

    pub fn new(name: String) -> Entity {
        let mut entity = Entity {
            name,
            health: 0,
            attributes: Attributes::default(),
            weapon: Weapon::default(),
        };

        entity.health = entity.max_health();
        entity
    }

    pub fn add_damage(&mut self, dmg: u32) {
        if dmg > self.health {
            self.health = 0;
        } else {
            self.health -= dmg;
        }
    }

    pub fn roll_hit(&self, other: &Entity) -> bool {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..21);
        roll >= other.get_ac()
    }

    pub fn roll_dmg(&self) -> u32 {
        self.weapon.get_damage()
    }

    pub fn get_ac(&self) -> i32 {
        10
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        self.weapon = weapon;
    }
}

#[derive(Debug)]
pub struct Attributes {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl Attributes {
    pub fn default() -> Attributes {
        Attributes {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}

#[derive(Debug)]
pub struct Modifiers {
    pub attack_modifier: u32,
    pub defense_modifier: u32,
}

impl Modifiers {
    pub fn default() -> Modifiers {
        Modifiers {
            attack_modifier: 0,
            defense_modifier: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Die {
    pub eyes: u8,
}

impl Die {
    pub fn throw(&self) -> u8 {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..self.eyes);
        roll
    }
}
