package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"path"
	"strconv"

	"Cyrkensia/server"
	"Cyrkensia/utils"
)

func GetHomeDir() string {
	dirname, err := os.UserHomeDir()
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}
	return dirname
}

func main() {
	// Flags
	port := flag.Int("p", 8080, "Port to listen on")
	root := flag.String("r", path.Join(GetHomeDir(), "Music"), "CDN root")
	locked := flag.Bool("L", false, "If set to true, files are password protected (default: false)")
	accessFile := flag.String("A", "ILLEGAL", "Path to .htaccess file to manage users (Required if -L is set to true)")
	name := flag.String("N", "Cyrkensia", "Name of the server")
	CDNroot := flag.String("R", "", "(Optional) Path for files endpoint in HTTP")
	icon := flag.String("i", "discordicon", "Icon to use for the server")
	flag.Parse()
	// Exit if no access file is specified but locked is true
	if *locked && *accessFile == "ILLEGAL" {
		log.Fatal("-L is set to true, but no -A flag was given")
		os.Exit(1)
	}
	// Config
	utils.Config = utils.ConfigType{
		Port:    *port,
		CDNroot: *CDNroot,
		Icon:    *icon,
		Root:    *root,
		Uuid:    utils.GetUUID(GetHomeDir()),
		Locked:  *locked,
		Name:    *name,
		Access:  *accessFile,
		TLS:     false,
		Key:     "",
		Cert:    "",
	}

	// Start server with endpoints
	http.HandleFunc("/hostinfo", server.HostinfoEndpoint)
	fmt.Printf("Starting server at port " + strconv.Itoa(*port) + "\n")
	if (flag.Args() != nil) && (len(flag.Args()) > 0) {
		certFile := flag.Args()[0]
		keyFile := flag.Args()[1]
		utils.Config.TLS = true
		utils.Config.Cert = certFile
		utils.Config.Key = keyFile
		if err := http.ListenAndServeTLS(":"+strconv.Itoa(*port), certFile, keyFile, nil); err != nil {
			log.Fatal(err)
			os.Exit(1)
		}
	} else {
		if err := http.ListenAndServe(":"+strconv.Itoa(*port), nil); err != nil {
			log.Fatal(err)
			os.Exit(1)
		}
	}
}
