package server

import (
	"Cyrkensia/utils"
	"path/filepath"

	"github.com/gofiber/fiber/v2"
	"github.com/tg123/go-htpasswd"
)

func FileServer(c *fiber.Ctx) error {
	filePath := filepath.Join(utils.Config.CDNpath, c.Params("directory"), c.Params("file"))
	if utils.FileExists(filePath) {
		return c.SendFile(filePath)
	} else {
		return ServeError404(c)
	}
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
