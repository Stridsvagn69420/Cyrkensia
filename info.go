package main

import (
	"runtime"

	"github.com/Stridsvagn69420/Cyrkensia/server"
	"github.com/Stridsvagn69420/Cyrkensia/utils"
	"github.com/Stridsvagn69420/pringo"
)

// Printer
var i1color pringo.Color = pringo.MagentaBright
var i2color pringo.Color = pringo.Magenta

func InfoMessage() {
	utils.Prnt.Println("Cyrkensia - Server for Azura and Shigure music repositories", i1color)
	info("Repository", server.SourceCode)
	info("License", server.License)
	info("Version", server.AppVersion)
	info("OS", runtime.GOOS)
	info("Arch", runtime.GOARCH)
	info("Go", runtime.Version())
}

func info(x string, y string) {
	utils.Prnt.Print(x+": ", i2color)
	utils.Prnt.Writeln(y)
}
