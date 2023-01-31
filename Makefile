up:
		docker compose -f docker/docker-compose.yaml up
down:
		[ ! -e file ] || rm -r ./api/tmp/*
		docker compose -f docker/docker-compose.yaml down
build:
		mkdir -p ./api/tmp
		docker compose -f docker/docker-compose.yaml build
