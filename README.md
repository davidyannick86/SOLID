# SOLID Principles

The SOLID principles are a set of five design guidelines that help developers create more maintainable, flexible, and scalable object-oriented software. Below is an in-depth explanation of each principle, along with Rust examples.

## 1. Single Responsibility Principle (SRP)

*A class or module should have only one reason to change.*

- Each type should focus on a single task or responsibility.
- Reduces coupling and makes code easier to test and maintain.

Example:

```rust
// Violation: Invoice handles calculation and printing
struct Invoice {
    items: Vec<Item>,
}

impl Invoice {
    fn calculate_total(&self) -> f32 { /* ... */ }
    fn print(&self) { /* ... */ }
}

// Refactored: Separate calculation and printing

struct InvoiceCalculator;

impl InvoiceCalculator {
    fn calculate(items: &[Item]) -> f32 { /* ... */ }
}

struct InvoicePrinter;

impl InvoicePrinter {
    fn print(invoice: &Invoice) { /* ... */ }
}
```

## 2. Open/Closed Principle (OCP)

*Software entities should be open for extension, but closed for modification.*

- You can add new behavior without altering existing code.
- Achieved through abstraction (traits) and polymorphism.

Example:

```rust
trait PaymentMethod {
    fn pay(&self, amount: f32);
}

struct CreditCard;

impl PaymentMethod for CreditCard {
    fn pay(&self, amount: f32) { /* credit card payment */ }
}

struct PayPal;

impl PaymentMethod for PayPal {
    fn pay(&self, amount: f32) { /* PayPal payment */ }
}

fn process_payment(method: &dyn PaymentMethod, amount: f32) {
    method.pay(amount);
}
```

## 3. Liskov Substitution Principle (LSP)

*Objects of a superclass should be replaceable with objects of a subclass without affecting correctness.*

- Subclasses must honor the contract of their base class (no unexpected behavior).
- Preconditions cannot be strengthened; postconditions cannot be weakened.

Example:

```rust
trait Bird {
    fn fly(&self);
}

struct Sparrow;

impl Bird for Sparrow {
    fn fly(&self) { /* can fly */ }
}

struct Ostrich;

// Ostrich cannot fly! Implementing Bird would violate LSP
```

## 4. Interface Segregation Principle (ISP)

*Clients should not be forced to depend on interfaces they do not use.*

- Split large interfaces into smaller, more specific ones.
- Implement only the methods needed.

Example:

```rust
trait Printer {
    fn print(&self);
}

trait Scanner {
    fn scan(&self);
}

struct MultiFunctionDevice;

impl Printer for MultiFunctionDevice {
    fn print(&self) { /*...*/ }
}

impl Scanner for MultiFunctionDevice {
    fn scan(&self) { /*...*/ }
}
```

## 5. Dependency Inversion Principle (DIP)

*High-level modules should not depend on low-level modules; both should depend on abstractions.*

- Abstractions should not depend on details; details should depend on abstractions.
- Use dependency injection to provide implementations at runtime.

### Examples of Dependency Injection

#### 1. Static Dispatch (Generics)

```rust
trait Repository {
    fn save(&self, data: &Data);
}

struct DatabaseRepository;

impl Repository for DatabaseRepository {
    fn save(&self, data: &Data) { /* write to DB */ }
}

struct BusinessLogic<T: Repository> {
    repo: T,
}

impl<T: Repository> BusinessLogic<T> {
    fn new(repo: T) -> Self { Self { repo } }
    fn execute(&self, data: &Data) {
        // high-level logic
        self.repo.save(data);
    }
}
```

#### 2. Dynamic Dispatch (Trait Objects)

```rust
trait Repository {
    fn save(&self, data: &Data);
}

struct DatabaseRepository;

impl Repository for DatabaseRepository {
    fn save(&self, data: &Data) { /* write to DB */ }
}

struct BusinessLogicDyn {
    repo: Box<dyn Repository>,
}

impl BusinessLogicDyn {
    fn new(repo: Box<dyn Repository>) -> Self { Self { repo } }
    fn execute(&self, data: &Data) {
        // high-level logic
        self.repo.save(data);
    }
}
```

---
