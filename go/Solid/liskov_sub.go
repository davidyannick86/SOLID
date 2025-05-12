package solid

import "fmt"

type Animal interface {
	MakeNoise() string
}

type Dog struct{}

func (d *Dog) MakeNoise() string {
	return "Woof!"
}

type Cat struct{}

func (c *Cat) MakeNoise() string {
	return "Meow!"
}

func AnimalNoise(a Animal) {
	fmt.Printf("Animal noise: %s\n", a.MakeNoise())
}
