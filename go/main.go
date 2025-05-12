package main

import (
	solid "github.com/davidyannick/solid/Solid"
)

func main() {
	// Single Responsibility Principle
	var user *solid.User
	user = user.NewUser("John Doe", 30)

	var userDisplay solid.UserDisplay
	userDisplay.Display(user)

	// Open/Closed Principle
	var circle solid.Circle
	var rectangle solid.Rectangle

	circle = solid.Circle{
		Radius: 5,
	}

	rectangle = solid.Rectangle{
		Width:  4,
		Height: 6,
	}

	solid.PrintArea(&circle)
	solid.PrintArea(&rectangle)

	// Liskov Substitution Principle
	dog := &solid.Dog{}
	cat := &solid.Cat{}

	solid.AnimalNoise(dog)
	solid.AnimalNoise(cat)

	// Interface Segregation Principle
	printer1 := &solid.SimplePrinter{}
	printer2 := &solid.MultiFunctionPrinter{}

	solid.UsePrinter(printer1)
	solid.UsePrinter(printer2)

	// Dependency Inversion Principle

	sqlDB := solid.SqlDatabase{}
	noSQL := solid.NoSqlDatabase{}

	app1 := solid.Application{
		Db: &sqlDB,
	}

	app2 := solid.Application{
		Db: &noSQL,
	}

	app1.SaveData("SQL Data")
	app2.SaveData("NoSQL Data")

}
