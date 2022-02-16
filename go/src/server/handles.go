package server

import (
	"log"
	"net/http"
)

func ServerError500(w http.ResponseWriter, r *http.Request, err error) {
	if err != nil {
		log.Fatal(err)
		http.Error(w, "500 Internal Server Error.", http.StatusInternalServerError)
		return
	}
}
