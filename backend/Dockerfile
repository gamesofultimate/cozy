FROM debian:stable-slim

WORKDIR /opt/app

#RUN apt-get update -y && apt-get install -y ca-certificates
#RUN dpkg-reconfigure -p critical ca-certificates

COPY ./backend/.env.staging ./.env
COPY ./backend/Rocket.toml ./dist/Rocket.toml
COPY ./backend/backend_server ./dist/backend_server

CMD ["/opt/app/dist/backend_server"]
