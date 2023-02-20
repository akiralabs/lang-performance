package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"

	"github.com/labstack/echo/v4"
	_ "github.com/lib/pq"
)

const (
	host     = "localhost"
	port     = 5432
	user     = "postgres"
	password = "postgres"
	dbname   = "goods"
)

var db *sql.DB

type Good struct {
	ID          int64  `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Price       int    `json:"price"`
}

func getAllGoods(c echo.Context, stmt *sql.Stmt) error {
	rows, err := stmt.Query()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, err)
	}
	defer rows.Close()

	goods := make([]Good, 0)
	for rows.Next() {
		var good Good
		if err := rows.Scan(&good.ID, &good.Name, &good.Description, &good.Price); err != nil {
			log.Fatal(err)
		}

		goods = append(goods, good)
	}

	return c.JSON(http.StatusOK, goods)
}

func main() {
	psqlInfo := fmt.Sprintf("host=%s port=%d user=%s "+
		"password=%s dbname=%s sslmode=disable",
		host, port, user, password, dbname)
	var err error
	db, err = sql.Open("postgres", psqlInfo)
	if err != nil {
		log.Fatal(err)
	}
	db.SetMaxOpenConns(50)

	e := echo.New()

	stmt, err := db.Prepare("SELECT id, name, description, price FROM goods")
	if err != nil {
		panic("Error creating prepared statement")
	}

	// Routes
	e.GET("/goods", func(c echo.Context) error {
		return getAllGoods(c, stmt)
	})

	// Start server
	e.Logger.Fatal(e.Start(":8080"))
}
