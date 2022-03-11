package utils

import (
	"errors"
	"io/ioutil"
	"log"
	"os"
	"strings"
)

func GetHomeDir() string {
	dirname, err := os.UserHomeDir()
	if err != nil {
		log.Fatal(err)
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

func ListFiles(path string) ([]string, error) {
	files, err := ioutil.ReadDir(path)
	var fileList []string
	for _, f := range files {
		if !strings.HasPrefix(f.Name(), ".") && !f.IsDir() {
			fileList = append(fileList, f.Name())
		}
	}
	return fileList, err
}

func ArrayContains(arr []string, str string) bool {
	for _, a := range arr {
		if a == str {
			return true
		}
	}
	return false
}
