package server

import (
	"github.com/gofiber/fiber/v2"
)

func MainServer(c *fiber.Ctx) error {
	if c.BaseURL() == "/" {
		return c.Redirect("/hostinfo")
	}
	// TODO: Serve files
	return nil
}
