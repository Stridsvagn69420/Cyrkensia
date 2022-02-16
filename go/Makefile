install:
	go install src/cyrkensia.go

daemon:
	sh go-daemon.sh

windows:
	go build -o build/cyrkensia.exe src/cyrkensia.go

linux:
	go build -o build/cyrkensia src/cyrkensia.go

test:
	go test -v src/cyrkensia.go

run:
	go run src/cyrkensia.go