prod:
	@docker-compose --file docker-compose.yml up

dev:
	@docker-compose --file docker-compose.development.yml up --build & \
	cd backend && cargo watch -x run

db:
	@docker exec -it livs-stack_db_1 psql -U livs

nuke:
	@docker container prune -f && \
	docker image prune -f  && \
	docker volume prune -f  && \
	docker volume rm livs-stack_db-data -f

.PHONY: nuke run db dev prod
