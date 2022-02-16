package utils

type ConfigType struct {
	Port    int
	Root    string
	CDNroot string
	Icon    string
	Uuid    string
	Locked  bool
	Name    string
	Access  string
	TLS     bool
	Cert    string
	Key     string
}

var Config ConfigType
