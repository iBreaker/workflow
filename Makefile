.PHONY: bin

install:bin
	cp bin/workflow  ~/go/bin

bin:
	go build -o bin/workflow  main.go