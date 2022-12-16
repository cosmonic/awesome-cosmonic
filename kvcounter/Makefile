# Contains some helper utilities for updating projects

update-kvcounter: ## Generates a new kvcounter project from rust, builds, and copies the compiled file to rust/build
	cosmo new actor --path rust kvcounter
	cosmo build -p kvcounter
	mkdir -p ./rust/build
	cp kvcounter/build/kvcounter.wasm ./rust/build/kvcounter.wasm
	rm -rf kvcounter
	git add --force ./rust/build/kvcounter.wasm

help:  ## Display this help
        @awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
