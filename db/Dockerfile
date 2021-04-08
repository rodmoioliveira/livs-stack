FROM postgres:latest
ENV POSTGRES_DB livs
RUN mkdir /csv/
COPY init.sql /docker-entrypoint-initdb.d/
COPY ./csv /csv/
