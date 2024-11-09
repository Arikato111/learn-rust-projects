default: run

run:
	cargo run

watch:
	nodemon --exec cargo run -e rs,toml --signal SIGTERM

build:
	cargo build

# For Docker
up:
	docker compose up -d

down:
	docker compose down --rmi local
