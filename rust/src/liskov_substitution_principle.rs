/*
Les objets d'une sous-classe doivent pouvoir remplacer les objets de la classe parente sans altérer le comportement attendu du programme.
*/

pub trait Animal {
    fn make_sound(&self) -> String;
}

pub struct Dog;
pub struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

pub fn animal_sound(animal: &dyn Animal) {
    println!("Animal sound: {}", animal.make_sound());
}
