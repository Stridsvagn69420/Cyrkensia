package main

import (
	"Cyrkensia/utils"
	"flag"
	"path/filepath"
)

func main() {
	// ------ Flags ------
	configPath := flag.String("Config", filepath.Join(utils.GetHomeDir(), ".config/cyrkensia/config.json"), "Config file location")
	// Listening address and port and file directory
	flag.StringVar(&utils.Config.BindAddr, "Bind", utils.Config.BindAddr, "IP address to bind to")
	flag.IntVar(&utils.Config.Port, "Port", utils.Config.Port, "Port to listen on")
	flag.StringVar(&utils.Config.CDNpath, "Files", utils.Config.CDNpath, "Folder holding all the audio files")
	// Appearance and UUID
	flag.StringVar(&utils.Config.Name, "Name", utils.Config.Name, "Name to represent the server")
	flag.StringVar(&utils.Config.Icon, "Icon", utils.Config.Icon, "Icon key to represent the server")
	flag.StringVar(&utils.Config.Uuid, "Uuid", utils.Config.Uuid, "UUID")
	// User access
	flag.StringVar(&utils.Config.Access, "Htpasswd", utils.Config.Access, "Path to htpasswd file for password protection")
	flag.Parse()

	// ------ Config ------
	utils.LoadConfig(*configPath)
}
