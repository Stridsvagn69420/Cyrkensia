executable:
	cd src; go build -o ../build/ -ldflags="-s -w" cyrkensia.go

install:
	cd src; go install -ldflags="-s -w" cyrkensia.go

release:
	pwsh ./Release.ps1

run:
	@cd src; go run cyrkensia.go

help:
	@cd src; go run cyrkensia.go -h
