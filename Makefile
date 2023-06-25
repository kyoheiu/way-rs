dev:
	WAY_SECRET_KEY=test WAY_DOMAIN=localhost run

build:
	sudo docker build --tag=kyoheiudev/way-rs:$(VER) .

push:
	sudo docker push kyoheiudev/way-rs:$(VER)

remove:
	sudo docker compose down --remove-orphans
	sudo docker rm $(sudo docker ps -a -q) -f
	sudo docker rmi $(sudo docker images -a -q) -f
