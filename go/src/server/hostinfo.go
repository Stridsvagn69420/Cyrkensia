package server

import (
	"encoding/json"
	"io/ioutil"
	"os"
	"path"
	"strconv"

	"github.com/Stridsvagn69420/Cyrkensia/go/src/utils"

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
	Cover string      `json:"cover"`
	Dir   string      `json:"dir"`
	Name  string      `json:"name"`
	Files []MusicFile `json:"files"`
}
type MusicFile struct {
	Name string `json:"name"`
	Size int    `json:"size"`
}

type metadata struct {
	Name  string `json:"name"`
	Cover string `json:"cover"`
}

func HostinfoEndpoint(c *fiber.Ctx) error {
	// Generate hostinfo
	hostinfo := Hostinfo{
		Name:      utils.Config.Name,
		Hosticon:  utils.Config.Icon,
		Uuid:      utils.Config.Uuid,
		Secured:   utils.Config.Locked,
		Root:      "",
		OriginURI: c.Protocol() + "://" + c.Hostname() + c.Path(),
	}
	hostinfo.Albums, hostinfo.Size = readAlbums(c, utils.Config.CDNpath)
	// send hostinfo response
	json, err := json.Marshal(hostinfo)
	ServerError500(c, err)
	c.Append("Content-Type", fiber.MIMEApplicationJSON)
	c.Append("Content-Length", strconv.Itoa(len(json)))
	return c.Status(fiber.StatusOK).Send(json)
}

// Hostinfo specific IO functions for reading out available songs and albums
func readAlbums(c *fiber.Ctx, pathdir string) ([]Album, int) {
	size := 0
	files, err := ioutil.ReadDir(pathdir)
	albums := make([]Album, len(files))
	if err != nil {
		ServerError500(c, err)
	}
	for i, file := range files {
		if file.IsDir() {
			albumpath := path.Join(pathdir, file.Name())
			if _, finderr := os.Stat(path.Join(albumpath, ".metadata.json")); finderr == nil {
				meta := readMetadata(c, albumpath)
				album := Album{
					Name:  meta.Name,
					Dir:   file.Name(),
					Cover: meta.Cover,
					Files: readFiles(c, albumpath, &size),
				}
				albums[i] = album
			}
		}
	}
	if len(albums) == 0 {
		albums = make([]Album, 0)
	}
	return albums, size
}

func readMetadata(c *fiber.Ctx, dirpath string) metadata {
	var metadata metadata
	file, err := ioutil.ReadFile(path.Join(dirpath, ".metadata.json"))
	if err != nil {
		ServerError500(c, err)
	}
	err = json.Unmarshal(file, &metadata)
	if err != nil {
		ServerError500(c, err)
	}
	return metadata
}

func readFiles(c *fiber.Ctx, dirpath string, size *int) []MusicFile {
	files, err := ioutil.ReadDir(dirpath)
	musicfiles := make([]MusicFile, len(files))
	if err != nil {
		ServerError500(c, err)
	}
	for i, file := range files {
		if !file.IsDir() && file.Name() != ".metadata.json" {
			filesize := int(file.Size())
			file := MusicFile{
				Name: file.Name(),
				Size: filesize,
			}
			*size += filesize
			musicfiles[i] = file
		}
	}
	return musicfiles
}
