prod: nuke
	@docker-compose --file docker-compose.production.yml up --build

dev: nuke
	@docker-compose --file docker-compose.development.yml up --build & \
	cd backend && cargo watch -x run & \
	cd frontend && cargo watch -x run & \
	cd assets && cargo watch -x run & \
	make sass-watch

# TODO: make your own docker file
sass-watch:
	@docker run --rm -v $(shell pwd)/assets/dev/scss:/sass/ -v $(shell pwd)/assets/static/css:/css/ michalklempa/dart-sass:latest

db:
	@docker exec -it livs-stack_db_1 psql -U livs

db-csv:
	@docker exec -it livs-stack_db_1 psql -U livs \
	-c "COPY authors TO '/csv/authors-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY formats TO '/csv/formats-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY customers TO '/csv/customers-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY genres TO '/csv/genres-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY inventory TO '/csv/inventory-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY languages TO '/csv/languages-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY measures TO '/csv/measures-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY publishers TO '/csv/publishers-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY reviews TO '/csv/reviews-dump.csv' WITH (FORMAT CSV, HEADER);" \
	-c "COPY titles TO '/csv/titles-dump.csv' WITH (FORMAT CSV, HEADER);"

nuke:
	@docker system prune --volumes -f

.PHONY: nuke run db dev prod db-csv sass-watch
