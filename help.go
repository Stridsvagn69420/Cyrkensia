package main

import (
	"fmt"

	"github.com/Stridsvagn69420/Cyrkensia/server"
	"github.com/Stridsvagn69420/Cyrkensia/utils"
	"github.com/Stridsvagn69420/pringo"
)

func HelpMessage() {
	utils.Prnt.Println("USAGE: cyrkensia [FLAGS]", color1)

	help(ConfigName, ConfigDesc, cfgpath)
	help(BindName, BindDesc, utils.Config.BindAddr)
	help(FilesName, FilesDesc, utils.Config.CDNpath)
	help(PortName, PortDesc, fmt.Sprint(utils.Config.Port))
	help(HtpasswdName, HtpasswdDesc, utils.Config.Access)
	help(UuidName, UuidDesc, utils.Config.Uuid)
	help(IconName, IconDesc, utils.Config.Icon)
	help(NameName, NameDesc, utils.Config.Name)
	help(PemName, PemDesc, utils.Config.Pem)
	help(KeyName, KeyDesc, utils.Config.Key)

	utils.Prnt.Print("See ", pringo.Blue)
	utils.Prnt.Print("cyrkensia info", pringo.BlueBright)
	utils.Prnt.Print(" or ", pringo.Blue)
	utils.Prnt.Print(server.SourceCode, pringo.BlueBright)
	utils.Prnt.Println(" for more!", pringo.Blue)
}

func help(x string, y string, z string) {
	utils.Prnt.Print("-"+x+": ", color2)
	utils.Prnt.Writeln(y)
	utils.Prnt.Print("Default: ", pringo.Blue)
	if z != "" {
		utils.Prnt.Writeln(z)
	} else {
		utils.Prnt.Writeln("None")
	}
	utils.Prnt.Writeln("")
}
