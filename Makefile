COMPOSE ?= docker compose

.PHONY: help build up up-detached down logs up-db logs-db down-db down-all

help:
	@echo "Available commands:"
	@echo "  build"
	@echo "  up"
	@echo "  up-detached"
	@echo "  logs"
	@echo "  up-db"
	@echo "  logs-db"

build:
	$(COMPOSE) build sedna-worker

up:
	$(COMPOSE) up --build sedna-worker

down:
	$(COMPOSE) down sedna-worker

up-detached:
	$(COMPOSE) up -d --build sedna-worker

logs:
	$(COMPOSE) logs sedna-worker -f

up-db:
	$(COMPOSE) up postgres -d

logs-db:
	$(COMPOSE) logs postgres -f

down-db:
	$(COMPOSE) down postgres

down-all:
	$(COMPOSE) down