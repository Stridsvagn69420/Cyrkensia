package utils

import (
	"encoding/json"
	"errors"
	"os"

	"github.com/Stridsvagn69420/pringo"
)

type ConfigType struct {
	Port     int    `json:"port"`
	CDNpath  string `json:"root"`
	BindAddr string `json:"bindAddr"`
	Icon     string `json:"hosticon"`
	Uuid     string `json:"uuid"`
	Locked   bool
	Name     string `json:"vendorName"`
	Access   string `json:"htpasswd"`
	Pem      string `json:"pemTLS"`
	Key      string `json:"keyTLS"`
}

var Config ConfigType

func LoadConfig(path string) {
	if _, err := os.Stat(path); errors.Is(err, os.ErrNotExist) {
		// File does not exist
		Prnt.Println("Config file does not exist! Please make sure that your given path is correct and that it points to a valid JSON file.", pringo.Red)
		Prnt.Println("See https://github.com/Stridsvagn69420/Cyrkensia/wiki/Installation#-config-file or the local README.md for more.", pringo.Yellow)
		os.Exit(1)
	} else {
		// Load config to config.Config from given json file in path
		jsonTxt, _ := os.ReadFile(path)
		tmpConfig := ConfigType{}
		json.Unmarshal([]byte(jsonTxt), &tmpConfig)

		// Assign values if empty
		assignIfEmpty(&Config.BindAddr, tmpConfig.BindAddr)
		assignIfEmpty(&Config.CDNpath, tmpConfig.CDNpath)
		assignIfEmpty(&Config.Icon, tmpConfig.Icon)
		assignIfEmpty(&Config.Uuid, tmpConfig.Uuid)
		assignIfEmpty(&Config.Name, tmpConfig.Name)
		assignIfEmpty(&Config.Access, tmpConfig.Access)
		assignIfEmpty(&Config.Pem, tmpConfig.Pem)
		assignIfEmpty(&Config.Key, tmpConfig.Key)

		// Special case for Locked and Port
		if Config.Port == 0 {
			Config.Port = tmpConfig.Port
		}
		if Config.Access == "" {
			Config.Locked = false
		} else {
			Config.Locked = true
		}
	}
}

func assignIfEmpty(variable *string, value string) {
	if *variable == "" {
		*variable = value
	}
}
