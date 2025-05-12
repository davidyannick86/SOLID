package solid

import "fmt"

type Database interface {
	Save(data string)
}

type SqlDatabase struct{}

func (db *SqlDatabase) Save(data string) {
	fmt.Println("Saving to SQL database:", data)
}

type NoSqlDatabase struct{}

func (db *NoSqlDatabase) Save(data string) {
	fmt.Println("Saving to NoSQL database:", data)
}

type Application struct {
	Db Database
}

func (app *Application) NewApplication(db Database) *Application {
	return &Application{Db: db}
}

func (app *Application) SaveData(data string) {
	app.Db.Save(data)
}
