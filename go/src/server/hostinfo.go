package server

import (
	"encoding/json"
	"fmt"
	"net/http"

	"Cyrkensia/utils"
)

type Hostinfo struct {
	Name      string  `json:"name"`
	Hosticon  string  `json:"hosticon"`
	Uuid      string  `json:"uuid"`
	Secured   bool    `json:"secured"`
	Size      int     `json:"size"`
	Root      string  `json:"root"`
	OriginURI string  `json:"originURI"`
	Albums    []Album `json:"albums"`
}
type Album struct {
	Cover string `json:"cover"`
	Dir   string `json:"dir"`
	Name  string `json:"name"`
	Files []File `json:"files"`
}
type File struct {
	Name string `json:"name"`
	Size int    `json:"size"`
}

func HostinfoEndpoint(w http.ResponseWriter, r *http.Request) {
	// security check
	if r.URL.Path != "/hostinfo" {
		http.Error(w, "404 not found.", http.StatusNotFound)
		return
	}
	if r.Method != "GET" {
		http.Error(w, "Method is not supported.", http.StatusNotFound)
		return
	}
	// Generate hostinfo
	var protocol string
	if utils.Config.Locked {
		protocol = "https"
	} else {
		protocol = "http"
	}
	size, err := utils.DirSize(utils.Config.CDNpath)
	ServerError500(w, r, err)
	// TODO: add albums
	hostinfo := Hostinfo{
		Name:      utils.Config.Name,
		Hosticon:  utils.Config.Icon,
		Uuid:      utils.Config.Uuid,
		Secured:   utils.Config.Locked,
		Root:      "",
		OriginURI: protocol + "://" + r.Host + r.URL.Path,
		Size:      int(size),
	}
	// send hostinfo response
	json, err := json.Marshal(hostinfo)
	ServerError500(w, r, err)
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Content-Length", fmt.Sprintf("%d", len(json)))
	w.Write(json)
}
