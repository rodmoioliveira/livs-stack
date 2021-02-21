run:
	docker-compose up --build

nuke:
	docker container prune -f && \
	docker image prune -f  && \
	docker volume prune -f  && \
	docker volume rm livs-stack_db-data -f

.PHONY: nuke
.PHONY: run
