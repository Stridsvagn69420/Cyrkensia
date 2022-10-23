package main

import (
	"github.com/Stridsvagn69420/Cyrkensia/utils"
	"github.com/Stridsvagn69420/pringo"
)

// Printer
var h1color pringo.Color = pringo.GreenBright
var h2color pringo.Color = pringo.Green

func HelpMessage() {

}

func help(x string, y string) {
	utils.Prnt.Print(x+": ", i2color)
	utils.Prnt.Writeln(y)
}
