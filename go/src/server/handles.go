package server

import (
	"Cyrkensia/utils"

	"github.com/Stridsvagn69420/pringo"

	"github.com/gofiber/fiber/v2"
)

func ServerError500(c *fiber.Ctx, err error) error {
	if err != nil {
		utils.Prnt.Errorln(err.Error(), pringo.Red)
		return c.Status(fiber.StatusInternalServerError).SendString("Internal Server Error")
	}
	return nil
}

func AuthError401(c *fiber.Ctx) error {
	c.Set("WWW-Authenticate", "Basic realm=Cyrkensia")
	return c.Status(fiber.StatusUnauthorized).SendString("Unauthorized")
}

func ServeError404(c *fiber.Ctx) error {
	return c.Status(fiber.StatusNotFound).SendString("Not Found")
}

func Forbidden403(c *fiber.Ctx) error {
	return c.Status(fiber.StatusForbidden).SendString("Forbidden")
}
