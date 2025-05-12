/*
Les modules de haut niveau ne doivent pas dépendre des modules de bas niveau. Les deux doivent dépendre d'abstractions (interfaces ou traits).
*/

pub trait Database {
    fn save(&self, data: &str);
}

pub struct SqlDatabase;
pub struct NoSqlDatabase;

impl Database for SqlDatabase {
    fn save(&self, data: &str) {
        println!("Saving data to SQL database: {}", data);
    }
}

impl Database for NoSqlDatabase {
    fn save(&self, data: &str) {
        println!("Saving data to NoSQL database: {}", data);
    }
}

// dynamic dispatch
pub struct Application {
    db: Box<dyn Database>,
}

impl Application {
    pub fn new(db: Box<dyn Database>) -> Self {
        Application { db }
    }

    pub fn save(&self, data: &str) {
        self.db.save(data);
    }
}

// static dispatch
pub struct Application2<D: Database> {
    db: D,
}

impl<D: Database> Application2<D> {
    pub fn new(db: D) -> Self {
        Self { db }
    }

    pub fn save(&self, data: &str) {
        self.db.save(data);
    }
}
