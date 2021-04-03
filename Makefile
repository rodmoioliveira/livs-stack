prod:
	@docker-compose --file docker-compose.yml up

dev:
	@docker-compose --file docker-compose.development.yml up --build & \
	cd backend && cargo watch -x run

db:
	@docker exec -it livs-stack_db_1 psql -U livs

nuke:
	@docker system prune --volumes

.PHONY: nuke run db dev prod
