package server

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

func ServerError500(c *fiber.Ctx, err error) {
	if err != nil {
		log.Fatal(err)
		c.Status(500).SendString("Internal Server Error")
	}
}
