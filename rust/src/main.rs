use dependency_inversion_principle::{Application, Application2, NoSqlDatabase, SqlDatabase};
use interface_segregation_principle::{MultiFunctionPrinter, SimplePrinter, use_printer};
use liskov_substitution_principle::{Cat, Dog, animal_sound};
use open_closed_principle::{Circle, Rectangle, print_area};
use single_responsibility_principle::{User, UserDisplay};

mod dependency_inversion_principle;
mod interface_segregation_principle;
mod liskov_substitution_principle;
mod open_closed_principle;
mod single_responsibility_principle;

fn main() {
    // 1 - Single Responsibility Principle (Principe de responsabilité unique)
    let user = User::new(String::from("John Doe"), 30);
    UserDisplay::display(&user);

    // 2 - Open/Closed Principle (Principe ouvert/fermé)
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    print_area(&circle);
    print_area(&rectangle);

    // 3 - Liskov Substitution Principle (Principe de substitution de Liskov)
    let dog = Dog;
    let cat = Cat;

    animal_sound(&dog);
    animal_sound(&cat);

    // 4 - Interface Segregation Principle (Principe de ségrégation des interfaces)
    let simple_printer = SimplePrinter;
    let multifunction_printer = MultiFunctionPrinter;

    use_printer(&simple_printer);
    use_printer(&multifunction_printer);

    // 5- Dependency Inversion Principle (Principe d'inversion des dépendances)

    // dynamic dispatch
    let sql_database = Box::new(SqlDatabase);
    let no_sql_database = Box::new(NoSqlDatabase);

    let app1 = Application::new(sql_database);
    let app2 = Application::new(no_sql_database);

    app1.save("User data");
    app2.save("User data");

    // static dispatch
    let sql_database2 = SqlDatabase;
    let no_sql_database2 = NoSqlDatabase;

    let app3 = Application2::new(sql_database2);
    let app4 = Application2::new(no_sql_database2);

    app3.save("User data");
    app4.save("User data");
}
