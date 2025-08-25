WORKSPACE_FLAGS = --workspace
CLIPPY_TARGETS = --all-targets

.PHONY: fmt lint patch minor test dump help

fmt: ## Checking formatting
	@cargo fmt --all -- --check

lint:
	@cargo clippy $(WORKSPACE_FLAGS) $(CLIPPY_TARGETS) -- -D warnings

test: ## Testing efx
	@cargo test -p efx-core
	@cargo test -p efx

test-all: ## Full workspace tests (integration + trybuild)
	@cargo test $(WORKSPACE_FLAGS)

check: fmt lint test-all ## Full set of pre-release checks

patch: ## Publish new patch version
	@make check
	@cargo release patch -p efx
	@cargo publish -p efx

minor: ## Publish new minor version
	@make check
	@cargo release minor -p efx
	@cargo publish -p efx

minor-core: ## Publish new efx-core minor version
	@cargo test -p efx-core
	@cargo fmt --all
	@cargo release minor -p efx-core
	@cargo publish -p efx-core

dump: ## Make dump of project
	@find ./ -type f \( -name "*.rs" -o -name "*.toml"  -o -name "*.md" \) \
		 ! -path "./target/*" \
		 -exec sh -c 'echo ">>> START {}"; cat "{}"; echo ">>> END {}"; echo ""' \; > efx_code.dump.txt

help: ## Outputs this help screen
	@grep -E '(^[a-zA-Z0-9\./_-]+:.*?##.*$$)|(^##)' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}{printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'
