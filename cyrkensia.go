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
	// User access and HTTPS
	flag.StringVar(&utils.Config.Access, "Htpasswd", utils.Config.Access, "Path to htpasswd file for password protection")
	flag.StringVar(&utils.Config.Pem, "Pem", utils.Config.Pem, "Path to the PEM file. Only needed for HTTPS.")
	flag.StringVar(&utils.Config.Key, "Key", utils.Config.Key, "Path to the KEY file. Only needed for HTTPS.")
	flag.Parse()

	// ------ Config ------
	utils.LoadConfig(*configPath)

	// ------ Server ------
	// Initialize server
	if utils.Config.Locked {
		if err := server.InitHtpasswdAuth(utils.Config.Access); err != nil {
			utils.Prnt.Println("An error occured while initializing HTTP-Basic Auth!", pringo.Red)
			utils.Prnt.Println("WARNING: Authorization will be disabled...", pringo.Yellow)
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
