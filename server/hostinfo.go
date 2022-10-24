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

type metadata struct {
	Name    string         `json:"name"`
	Cover   string         `json:"cover"`
	Authors []utils.Author `json:"authors"`
}

func HostinfoEndpoint(c *fiber.Ctx) error {
	// Generate hostinfo
	hostinfo := utils.Hostinfo{
		Name:      utils.Config.Name,
		Hosticon:  utils.Config.Icon,
		Uuid:      utils.Config.Uuid,
		Secured:   utils.Config.Locked,
		Root:      "",
		OriginURI: c.Protocol() + "://" + c.Hostname() + c.Path(),
		Owners:    utils.Config.Owners,
	}

	// Append dynamic fields
	hostinfo.Albums, hostinfo.Size = readAlbums(c, utils.Config.CDNpath)

	// Convert to JSON
	json, err := json.Marshal(hostinfo)
	ServerError500(c, err)
	// Set Headers
	c.Append("Content-Type", fiber.MIMEApplicationJSON)
	c.Append("Content-Length", strconv.Itoa(len(json)))

	// Send JSON response
	return c.Status(fiber.StatusOK).Send(json)
}

// Hostinfo specific IO functions for reading out available songs and albums
func readAlbums(c *fiber.Ctx, pathdir string) ([]utils.Album, int) {
	// Read files
	files, err := os.ReadDir(pathdir)
	if err != nil {
		ServerError500(c, err)
	}

	// Vars
	size := 0
	var albums []utils.Album

	// Read directories
	for _, file := range files {
		if file.IsDir() {
			albumpath := path.Join(pathdir, file.Name())
			// Extract metadata
			if _, finderr := os.Stat(path.Join(albumpath, ".metadata.json")); finderr == nil {
				meta := readMetadata(c, albumpath)
				album := utils.Album{
					Name:    meta.Name,
					Dir:     file.Name(),
					Cover:   meta.Cover,
					Files:   readFiles(c, albumpath, &size),
					Authors: meta.Authors,
				}
				albums = append(albums, album)
			}
		}
	}
	if len(albums) == 0 {
		albums = make([]utils.Album, 0)
	}

	// Return albums and size
	return albums, size
}

func readMetadata(c *fiber.Ctx, dirpath string) metadata {
	var metadata metadata
	// Read metadata
	file, err := os.ReadFile(path.Join(dirpath, ".metadata.json"))
	if err != nil {
		ServerError500(c, err)
	}

	// Parse JSON
	err = json.Unmarshal(file, &metadata)
	if err != nil {
		ServerError500(c, err)
	}

	// Retunr metadata
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

	// Return music files
	return musicfiles
}
