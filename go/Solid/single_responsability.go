package solid

import "fmt"

type User struct {
	Name string
	Age  int
}

func (u *User) NewUser(name string, age int) *User {
	return &User{
		Name: name,
		Age:  age,
	}
}

func (u *User) GetName() string {
	return u.Name
}

func (u *User) GetAge() int {
	return u.Age
}

type UserDisplay struct {
}

func (ud *UserDisplay) Display(user *User) {
	fmt.Printf("Name: %s, Age: %d\n", user.GetName(), user.GetAge())
}
