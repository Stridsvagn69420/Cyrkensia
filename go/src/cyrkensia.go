package main

import (
	"Cyrkensia/utils"
	"flag"
	"os"
	"path/filepath"
)

func main() {
	cwd, _ := os.Getwd()
	// ------ Flags ------
	flag.StringVar(&utils.Locations[3], "Config", filepath.Join(cwd, "config.json"), "Additional config file location")
	// Listening address and port and file directory
	flag.StringVar(&utils.Config.BindAddr, "BindAddr", utils.Config.BindAddr, "IP address to bind to")
	flag.IntVar(&utils.Config.Port, "Port", utils.Config.Port, "Port to listen on")
	flag.StringVar(&utils.Config.CDNpath, "Files", utils.Config.CDNpath, "Folder holding all the audio files")
	// Appearance and UUID
	flag.StringVar(&utils.Config.Name, "Name", utils.Config.Name, "Name to represent the server")
	flag.StringVar(&utils.Config.Icon, "Icon", utils.Config.Icon, "Icon key to represent the server")
	flag.StringVar(&utils.Config.Uuid, "Uuid", utils.Config.Uuid, "UUID or plain-text file containing the UUID")
	// User access
	flag.BoolVar(&utils.Config.Locked, "Locked", utils.Config.Locked, "Whether the server is password protected or not")
	flag.StringVar(&utils.Config.Access, "Htpasswd", utils.Config.Access, "Path to htpasswd file for password protection (only needed if Locked is true)")

	flag.Parse()
}
