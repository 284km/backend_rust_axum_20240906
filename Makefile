
db:
	docker-compose up

migrate:
	sqlx db create
	sqlx migrate run
