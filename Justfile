alias build := docker-build
alias run := docker-run

docker-build:
    docker-compose build

docker-run: docker-build
    docker-compose up
    docker-compose down
