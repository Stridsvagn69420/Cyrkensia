package server

import (
	"github.com/gofiber/fiber/v2"
)

func AgplHeaders(c *fiber.Ctx) error {
	c.Set("X-License", License)
	c.Set("X-License-URL", "https://www.gnu.org/licenses/agpl-3.0.html")
	c.Set("X-Source-Code", SourceCode)
	return c.Next()
}

func ServerHeader(c *fiber.Ctx) error {
	c.Set("Server", "Cyrkensia/"+AppVersion+" Fiber/"+fiber.Version)
	c.Set("Access-Control-Allow-Private-Network", "true")
	return c.Next()
}

var AppVersion string = "1.2.0"
var SourceCode string = "https://github.com/Stridsvagn69420/Cyrkensia"
var License string = "AGPL-3.0"
