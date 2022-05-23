package server

import (
	"github.com/Stridsvagn69420/Cyrkensia/go/utils"

	"encoding/base64"
	"net/url"
	"path/filepath"
	"strings"

	"github.com/gofiber/fiber/v2"
	"github.com/tg123/go-htpasswd"
)

func FileServer(c *fiber.Ctx) error {
	directory, err := url.QueryUnescape(c.Params("directory"))
	if err != nil {
		return ServerError500(c, err)
	}
	file, err := url.QueryUnescape(c.Params("file"))
	if err != nil {
		return ServerError500(c, err)
	}
	filePath := filepath.Join(utils.Config.CDNpath, directory, file)
	return sendIfExists(c, filePath)
}

func FileServerLocked(c *fiber.Ctx) error {
	userpass := c.GetReqHeaders()["Authorization"]
	if userpass == "" {
		return AuthError401(c)
	}
	decodedBytes, err := base64.StdEncoding.DecodeString(userpass[6:])
	ServerError500(c, err)
	decoded := strings.Split(string(decodedBytes), ":")
	if len(decoded) != 2 {
		return AuthError401(c)
	}
	if Auth.Match(decoded[0], decoded[1]) {
		return FileServer(c)
	}
	return AuthError401(c)
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

	// Hostinfo
	case "hostinfo":
		return HostinfoEndpoint(c)
	// License
	case "license":
		return c.Redirect("https://www.gnu.org/licenses/agpl-3.0.html", fiber.StatusMovedPermanently)

	// Album Index
	default:
		albumRoot := filepath.Join(utils.Config.CDNpath, route)
		if utils.FileExists(albumRoot) {
			fileList := utils.ListFiles(albumRoot)
			return c.JSON(fileList)
		} else {
			return ServeError404(c)
		}
	}
}

func sendIfExists(c *fiber.Ctx, filePath string) error {
	if utils.FileExists(filePath) {
		return c.SendFile(filePath)
	} else {
		return ServeError404(c)
	}
}
