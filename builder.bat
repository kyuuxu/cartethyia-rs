docker build -t wicked-waifus-builder:2.1.0-SNAPSHOT -f Dockerfile-builder .

docker build -t wicked-waifus-config-server:2.1.0-SNAPSHOT --build-arg MICROSERVICE=wicked-waifus-config-server -f Dockerfile-service .
docker build -t wicked-waifus-hotpatch-server:2.1.0-SNAPSHOT --build-arg MICROSERVICE=wicked-waifus-server -f Dockerfile-service .
docker build -t wicked-waifus-login-server:2.1.0-SNAPSHOT --build-arg MICROSERVICE=wicked-waifus-login-server -f Dockerfile-service .
docker build -t wicked-waifus-gateway-server:2.1.0-SNAPSHOT --build-arg MICROSERVICE=wicked-waifus-gateway-server -f Dockerfile-service .
docker build -t wicked-waifus-game-server:2.1.0-SNAPSHOT --build-arg MICROSERVICE=wicked-waifus-game-server -f Dockerfile-service .

docker rmi wicked-waifus-builder:2.1.0-SNAPSHOT

: Persistence for the application
: docker volume create wicked-waifus-postgres-vol