COMPOSE ?= docker compose

.PHONY: help build up up-detached logs up-db logs-db

help:
	@echo "Available commands:"
	@echo "  build"
	@echo "  up"
	@echo "  up-detached"
	@echo "  logs"
	@echo "  up-db"
	@echo "  logs-db"

build:
	$(COMPOSE) build --no-cache sedna-worker

up:
	$(COMPOSE) up sedna-worker

up-detached:
	$(COMPOSE) up sedna-worker -d

logs:
	$(COMPOSE) logs sedna-worker -f

up-db:
	$(COMPOSE) up postgres -d

logs-db:
	$(COMPOSE) logs postgres -f