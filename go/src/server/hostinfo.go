package server

import (
	"encoding/json"
	"strconv"

	"Cyrkensia/utils"

	"github.com/gofiber/fiber/v2"
)

type Hostinfo struct {
	Name      string  `json:"name"`
	Hosticon  string  `json:"hosticon"`
	Uuid      string  `json:"uuid"`
	Secured   bool    `json:"secured"`
	Size      int     `json:"size"`
	Root      string  `json:"root"`
	OriginURI string  `json:"originURI"`
	Albums    []Album `json:"albums"`
}
type Album struct {
	Cover string `json:"cover"`
	Dir   string `json:"dir"`
	Name  string `json:"name"`
	Files []File `json:"files"`
}
type File struct {
	Name string `json:"name"`
	Size int    `json:"size"`
}

func HostinfoEndpoint(c *fiber.Ctx) error {
	// Generate hostinfo
	size, err := utils.DirSize(utils.Config.CDNpath)
	ServerError500(c, err)
	// TODO: add albums
	hostinfo := Hostinfo{
		Name:      utils.Config.Name,
		Hosticon:  utils.Config.Icon,
		Uuid:      utils.Config.Uuid,
		Secured:   utils.Config.Locked,
		Root:      "",
		OriginURI: c.Protocol() + "://" + c.Hostname() + c.Path(),
		Size:      int(size),
	}
	// send hostinfo response
	json, err := json.Marshal(hostinfo)
	ServerError500(c, err)
	c.Append("Content-Type", fiber.MIMEApplicationJSON)
	c.Append("Content-Length", strconv.Itoa(len(json)))
	return c.Status(fiber.StatusOK).Send(json)
}
