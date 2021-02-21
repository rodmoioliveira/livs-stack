run:
	docker-compose up --build

db:
	docker exec -it livs-stack_db_1 psql -U livs

nuke:
	docker container prune -f && \
	docker image prune -f  && \
	docker volume prune -f  && \
	docker volume rm livs-stack_db-data -f

.PHONY: nuke
.PHONY: run
.PHONY: db
