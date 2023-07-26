dev:
	rm -rf ./static
	mkdir ./static
	cd svelte && npm run build
	cp -r ./svelte/dist/* ./static
	mv ./static/font ./static/assets/font
	WAY_SECRET_KEY=test WAY_DOMAIN=localhost WAY_NETWORK=ldap://localhost:3890 cargo run

build:
	sudo docker build --tag=kyoheiudev/way-rs:$(VER) .

push:
	sudo docker push kyoheiudev/way-rs:$(VER)

remove:
	sudo docker compose down --remove-orphans
	sudo docker rm $(sudo docker ps -a -q) -f
	sudo docker rmi $(sudo docker images -a -q) -f
