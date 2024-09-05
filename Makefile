
db:
	docker-compose up

migrate:
	sqlx db create
	sqlx migrate run

dbc:
	psql -Uadmin -h127.0.0.1 -p5432 -dsampledb

test:
	cargo test

