package main

import (
	"runtime"

	"github.com/Stridsvagn69420/Cyrkensia/server"
	"github.com/Stridsvagn69420/Cyrkensia/utils"
	"github.com/Stridsvagn69420/pringo"
)

// Printer
var color1 pringo.Color = pringo.MagentaBright
var color2 pringo.Color = pringo.Magenta

func InfoMessage() {
	utils.Prnt.Println("Cyrkensia - Server for Azura and Shigure music repositories", color1)
	info("Repository", server.SourceCode)
	info("License", server.License)
	info("Version", server.AppVersion)
	info("OS", runtime.GOOS)
	info("Arch", runtime.GOARCH)
	info("Go", runtime.Version())
}

func info(x string, y string) {
	utils.Prnt.Print(x+": ", color2)
	utils.Prnt.Writeln(y)
}
