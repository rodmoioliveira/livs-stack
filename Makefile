prod: nuke
	@docker-compose --file docker-compose.production.yml up --build

dev: nuke
	@docker-compose --file docker-compose.development.yml up --build & \
	cd backend && cargo watch -x run

db:
	@docker exec -it livs-stack_db_1 psql -U livs

db-csv:
	@docker exec -it livs-stack_db_1 psql -U livs \
	-c "COPY titles TO '/csv/titles-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY genres TO '/csv/genres-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY publishers TO '/csv/publishers-dump.csv' WITH (FORMAT CSV, HEADER);"

nuke:
	@docker system prune --volumes -f

.PHONY: nuke run db dev prod db-csv
