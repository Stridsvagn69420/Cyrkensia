package server

import (
	"github.com/gofiber/fiber/v2"
)

func AgplHeaders(c *fiber.Ctx) error {
	c.Set("X-License", "AGPL-3.0")
	c.Set("X-License-URL", "https://www.gnu.org/licenses/agpl-3.0.html")
	c.Set("X-Source-Code", "https://github.com/Stridsvagn69420/Cyrkensia")
	return c.Next()
}

func ServerHeader(c *fiber.Ctx) error {
	c.Set("Server", "Cyrkensia/"+AppVersion+" Fiber/"+fiber.Version)
	return c.Next()
}

var AppVersion string = "1.1.0"
