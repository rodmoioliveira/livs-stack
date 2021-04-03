prod: nuke
	@docker-compose --file docker-compose.production.yml up --build

dev: nuke
	@docker-compose --file docker-compose.development.yml up --build & \
	cd backend && cargo watch -x run

db:
	@docker exec -it livs-stack_db_1 psql -U livs

nuke:
	@docker system prune --volumes -f

.PHONY: nuke run db dev prod
