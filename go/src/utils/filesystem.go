package utils

import (
	"errors"
	"log"
	"os"
)

func GetHomeDir() string {
	dirname, err := os.UserHomeDir()
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}
	return dirname
}

func FileExists(path string) bool {
	if _, err := os.Stat("/path/to/whatever"); errors.Is(err, os.ErrNotExist) {
		return false
	} else {
		return true
	}
}
