package main

import (
	"Cyrkensia/server"
	"Cyrkensia/utils"
	"flag"
	"path/filepath"
	"strconv"

	"github.com/gofiber/fiber/v2"
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
	app := fiber.New()
	app.Use(logger.New(logger.Config{
		Format:     "[${ip}]:${port} ${status} - ${method} ${path}\n",
		TimeZone:   "UTC",
		TimeFormat: "2006-01-02 15:04:05",
	}))
	// Routes
	app.Get("/", server.HostinfoEndpoint)
	app.Get("/hostinfo", server.HostinfoEndpoint)
	app.Get("/hostinfo.json", server.HostinfoEndpoint)

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
