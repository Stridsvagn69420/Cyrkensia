package utils

import "path/filepath"

type ConfigType struct {
	Port     int
	CDNpath  string
	BindAddr string
	Icon     string
	Uuid     string
	Locked   bool
	Name     string
	Access   string
}

var Config ConfigType

var Locations []string = []string{
	"/etc/cyrkensia/config.json",
	filepath.Join(GetHomeDir(), ".cyrkensia/config.json"),
	filepath.Join(GetHomeDir(), ".config/cyrkensia/config.json"),
	"",
}
