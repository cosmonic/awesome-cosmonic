# Contains some helper utilities for updating projects

update-hello: ## Generates a new hello project from rust, builds, and copies the compiled file to rust/build
	cosmo new actor --path rust hello
	cd hello && cosmo build
	mkdir -p ./rust/build
	cp hello/build/hello.wasm ./rust/build/hello.wasm
	rm -rf hello
	git add --force ./rust/build/hello.wasm

help:  ## Display this help
        @awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
