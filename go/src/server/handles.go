package server

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

func ServerError500(c *fiber.Ctx, err error) {
	if err != nil {
		log.Fatal(err)
		c.Status(fiber.StatusInternalServerError).SendString("Internal Server Error")
	}
}

func AuthError401(c *fiber.Ctx) {
	c.Status(fiber.StatusUnauthorized).SendString("Unauthorized")
}

func ServeError404(c *fiber.Ctx) {
	c.Status(fiber.StatusNotFound).SendString("Not Found")
}
