package server

import (
	"encoding/json"
	"io/fs"
	"os"
	"path"
	"strconv"

	"github.com/Stridsvagn69420/Cyrkensia/utils"

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
	Cover string   `json:"cover"`
	Dir   string   `json:"dir"`
	Name  string   `json:"name"`
	Files []string `json:"files"`
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
	files, err := os.ReadDir(pathdir)
	if err != nil {
		ServerError500(c, err)
	}
	size := 0
	var albums []Album
	for _, file := range files {
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
				albums = append(albums, album)
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
	file, err := os.ReadFile(path.Join(dirpath, ".metadata.json"))
	if err != nil {
		ServerError500(c, err)
	}
	err = json.Unmarshal(file, &metadata)
	if err != nil {
		ServerError500(c, err)
	}
	return metadata
}

func readFiles(c *fiber.Ctx, dirpath string, size *int) []string {
	// Read files
	entries, err := os.ReadDir(dirpath)
	if err != nil {
		ServerError500(c, err)
	}
	// Get file info
	files := make([]fs.FileInfo, 0, len(entries))
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			ServerError500(c, err)
		}
		files = append(files, info)
	}

	// Parse metadata
	musicfiles := make([]string, len(files)-1)
	for i, file := range files {
		if !file.IsDir() && file.Name() != ".metadata.json" {
			*size += int(file.Size())
			musicfiles[i-1] = file.Name()
		}
	}
	return musicfiles
}
