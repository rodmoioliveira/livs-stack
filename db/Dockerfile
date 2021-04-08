FROM postgres:latest
ENV POSTGRES_DB livs
RUN mkdir /csv/
COPY . /docker-entrypoint-initdb.d/
