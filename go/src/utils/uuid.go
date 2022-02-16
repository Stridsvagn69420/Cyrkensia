package utils

import (
	"os"
	"path"

	"github.com/google/uuid"
)

func GetUUID(homedir string) string {
	var ID string
	storage := path.Join(homedir, ".uuid")
	if _, err := os.Stat(storage); os.IsNotExist(err) {
		ID = uuid.New().String()
		os.WriteFile(storage, []byte(ID), 0644)
	} else {
		content, err := os.ReadFile(storage)
		if err != nil {
			panic(err)
		}
		ID = string(content)
	}
	return ID
}
