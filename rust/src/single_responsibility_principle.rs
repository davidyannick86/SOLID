/*
Une classe (ou un module) ne devrait avoir qu'une seule responsabilité, c'est-à-dire une seule raison de changer.
*/

// Structure pour gérer les utilisateurs
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        User { name, age }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }
}

// Structure pour gérer l'affichage des utilisateurs

pub struct UserDisplay;

impl UserDisplay {
    pub fn display(user: &User) {
        println!("User: {}, Age: {}", user.get_name(), user.get_age());
    }
}
