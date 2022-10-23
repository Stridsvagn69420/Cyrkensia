package main

import (
	"flag"
	"os"
	"path/filepath"
	"strconv"

	"github.com/Stridsvagn69420/Cyrkensia/server"
	"github.com/Stridsvagn69420/Cyrkensia/utils"
	"github.com/Stridsvagn69420/pringo"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
)

const BindDesc string = "IP address to bind to"
const PortDesc string = "Port to listen on"
const FilesDesc string = "Folder holding all the audio files"
const NameDesc string = "Name to represent the server"
const IconDesc string = "Icon key to represent the server"
const UuidDesc string = "UUIDv4 of the server"
const HtpasswdDesc string = "Path to htpasswd file for password protection"
const PemDesc string = "Path to the PEM file. Only needed for HTTPS."
const KeyDesc string = "Path to the KEY file. Only needed for HTTPS."
const ConfigDesc string = "Config file location"

var cfgpath string = filepath.Join(utils.GetHomeDir(), ".config/cyrkensia/config.json")

const BindName string = "Bind"
const PortName string = "Port"
const FilesName string = "Files"
const NameName string = "Name"
const IconName string = "Icon"
const UuidName string = "Uuid"
const HtpasswdName string = "Htpasswd"
const PemName string = "Pem"
const KeyName string = "Key"
const ConfigName string = "Config"

func main() {
	// Load default config
	utils.LoadConfig(cfgpath)

	// Info and Help message
	if len(os.Args) > 1 {
		switch os.Args[1] {
		case "info", "version", "-V", "--version":
			InfoMessage()
			os.Exit(0)

		case "help", "-h", "--help":
			HelpMessage()
			os.Exit(0)
		}
	}

	// ------ Flags ------
	configPath := flag.String("Config", cfgpath, ConfigDesc)
	// Listening address and port and file directory
	flag.StringVar(&utils.Config.BindAddr, BindName, utils.Config.BindAddr, BindDesc)
	flag.IntVar(&utils.Config.Port, PortName, utils.Config.Port, PortDesc)
	flag.StringVar(&utils.Config.CDNpath, FilesName, utils.Config.CDNpath, FilesDesc)
	// Appearance and UUID
	flag.StringVar(&utils.Config.Name, NameName, utils.Config.Name, NameDesc)
	flag.StringVar(&utils.Config.Icon, IconName, utils.Config.Icon, IconDesc)
	flag.StringVar(&utils.Config.Uuid, UuidName, utils.Config.Uuid, UuidDesc)
	// User access and HTTPS
	flag.StringVar(&utils.Config.Access, HtpasswdName, utils.Config.Access, HtpasswdDesc)
	flag.StringVar(&utils.Config.Pem, PemName, utils.Config.Pem, PemDesc)
	flag.StringVar(&utils.Config.Key, KeyName, utils.Config.Key, KeyDesc)

	// Parse flags and load config
	flag.Parse()
	utils.LoadConfig(*configPath)

	// ------ Server ------
	// Initialize server
	if utils.Config.Locked {
		if err := server.InitHtpasswdAuth(utils.Config.Access); err != nil {
			utils.Prnt.Println("An error occured while initializing HTTP-Basic Auth!", pringo.Red)
			utils.Prnt.Println("WARNING: Server will shut down!", pringo.Yellow)
			os.Exit(1)
		}
	}
	app := fiber.New()
	// Middleware
	app.Use(logger.New(logger.Config{
		Format:     "[${ip}]:${port} ${status} - ${method} ${path}\n",
		TimeZone:   "UTC",
		TimeFormat: "2006-01-02 15:04:05",
	}))
	app.Use(cors.New(cors.Config{
		AllowOrigins:     "*",
		AllowHeaders:     "Origin, Content-Type, Accept, DNT, User-Agent, X-Requested-With, If-Modified-Since, Cache-Control, Range, Content-Length, Accept-Language, Accept-Encoding, Connection, Access-Control-Allow-Origin",
		AllowCredentials: true,
		AllowMethods:     "GET, HEAD, OPTIONS",
		ExposeHeaders:    "Content-Length, Content-Range",
	}))
	app.Use(server.AgplHeaders)
	app.Use(server.ServerHeader)

	// Routes
	app.Get("/", server.HostinfoEndpoint)
	app.Get("/:route", server.RouteEndpoint)
	if utils.Config.Locked {
		app.Get("/:directory/:file", server.FileServerLocked)
	} else {
		app.Get("/:directory/:file", server.FileServer)
	}

	// Start
	if utils.Config.Key != "" && utils.Config.Pem != "" && utils.FileExists(utils.Config.Key) && utils.FileExists(utils.Config.Pem) {
		app.ListenTLS(
			utils.Config.BindAddr+":"+strconv.Itoa(utils.Config.Port),
			utils.Config.Pem,
			utils.Config.Key,
		)
	} else {
		app.Listen(utils.Config.BindAddr + ":" + strconv.Itoa(utils.Config.Port))
	}
}
