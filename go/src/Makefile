install:
	go install cyrkensia.go

daemon:
	sh go-daemon.sh

windows:
	go build -o ../build/cyrkensia.exe cyrkensia.go

linux:
	go build -o ../build/cyrkensia cyrkensia.go

test:
	go test -v cyrkensia.go

run:
	go run cyrkensia.go

help:
	go run cyrkensia.go -h
