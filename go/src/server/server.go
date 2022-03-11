package server

import (
	"Cyrkensia/utils"
	"path/filepath"

	"github.com/gofiber/fiber/v2"
)

func FileServer(c *fiber.Ctx) error {
	return c.SendFile(filepath.Join(utils.Config.CDNpath, c.Params("directory"), c.Params("file")))
}

func FileServerLocked(c *fiber.Ctx) error {
	return FileServer(c)
}
