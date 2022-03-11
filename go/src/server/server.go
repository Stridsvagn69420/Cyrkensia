package server

import (
	"Cyrkensia/utils"
	"path/filepath"

	"github.com/gofiber/fiber/v2"
	"github.com/tg123/go-htpasswd"
)

func FileServer(c *fiber.Ctx) error {
	filePath := filepath.Join(utils.Config.CDNpath, c.Params("directory"), c.Params("file"))
	return sendIfExists(c, filePath)
}

func FileServerLocked(c *fiber.Ctx) error {
	return FileServer(c)
}

var Auth *htpasswd.File

func InitHtpasswdAuth(htpath string) error {
	var err error
	Auth, err = htpasswd.New(htpath, htpasswd.DefaultSystems, nil)
	return err
}

func RouteEndpoint(c *fiber.Ctx) error {
	route := c.Params("route")
	switch route {
	// Favicon
	case "favicon.ico":
		return sendIfExists(c, filepath.Join(utils.Config.CDNpath, "favicon.ico"))

	// Source Code
	case "github":
		return c.Redirect("https://github.com/Stridsvagn69420/Cyrkensia", fiber.StatusMovedPermanently)

	// License
	case "license":
		return c.Redirect("https://www.gnu.org/licenses/agpl-3.0.de.html", fiber.StatusMovedPermanently)

	// Album Index
	default:
		albumRoot := filepath.Join(utils.Config.CDNpath, route)
		fileList, err := utils.ListFiles(albumRoot)
		if err != nil {
			return ServeError404(c)
		}
		return c.JSON(fileList)
	}
}

func sendIfExists(c *fiber.Ctx, filePath string) error {
	if utils.FileExists(filePath) {
		return c.SendFile(filePath)
	} else {
		return ServeError404(c)
	}
}
