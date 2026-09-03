pub trait CharacterClass {
    type Weapon: Weapon;
    fn create_weapon() -> Self::Weapon;
}

trait Weapon {
    fn attack(&self);
}

pub struct Warrior;     // воин
pub struct Mage;        // маг

pub struct Sword;       // меч
pub struct Staff;       // посох

impl Weapon for Sword {
    fn attack(&self) {
        println!("Удар мечом!");
    }
}

impl Weapon for Staff {
    fn attack(&self) {
        println!("Волшебное заклинание посохом!");
    }
}

impl CharacterClass for Warrior {
    type Weapon = Sword;
    fn create_weapon() -> Self::Weapon {
        Sword
    }
}

impl CharacterClass for Mage {
    type Weapon = Staff;
    fn create_weapon() -> Self::Weapon {
        Staff
    }
}

pub fn attack<C: CharacterClass>() {
    let weapon = C::create_weapon();
    weapon.attack();
}