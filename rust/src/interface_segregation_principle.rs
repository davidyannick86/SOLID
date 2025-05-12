/*
Les clients ne devraient pas être forcés de dépendre d'interfaces qu'ils n'utilisent pas. Il vaut mieux avoir plusieurs petites interfaces spécifiques qu'une seule grande interface générale.
*/

pub trait Printer {
    fn print(&self);
}

pub trait Scanner {
    fn scan(&self);
}
pub struct SimplePrinter;
pub struct MultiFunctionPrinter;

impl Printer for SimplePrinter {
    fn print(&self) {
        println!("Printing from SimplePrinter");
    }
}

impl Printer for MultiFunctionPrinter {
    fn print(&self) {
        println!("Printing from MultiFunctionPrinter");
    }
}

impl Scanner for MultiFunctionPrinter {
    fn scan(&self) {
        println!("Scanning from MultiFunctionPrinter");
    }
}

pub fn use_printer(printer: &dyn Printer) {
    printer.print();
}
