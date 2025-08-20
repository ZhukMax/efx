.PHONY: patch minor test dump help

patch: ## Publish new patch version
	@cargo release patch

minor: ## Publish new minor version
	@cargo release minor

test: ## Testing efx
	@cargo test -p efx-core
	@cargo test -p efx

dump: ## Make dump of project
	@find ./ -type f \( -name "*.rs" -o -name "*.toml"  -o -name "*.md" \) \
		 ! -path "./target/*" \
		 -exec sh -c 'echo ">>> START {}"; cat "{}"; echo ">>> END {}"; echo ""' \; > efx_code.dump.txt

help: ## Outputs this help screen
	@grep -E '(^[a-zA-Z0-9\./_-]+:.*?##.*$$)|(^##)' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}{printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'
