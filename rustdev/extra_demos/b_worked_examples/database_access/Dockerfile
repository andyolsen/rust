# Pull MySQL from Docker Hub (if not already in local Docker registry).
FROM mysql:8.0.27

# MySQL will run on port 3306 within the container.
EXPOSE 3306

# Set an environment variable, which MySQL will look for.
ENV MYSQL_ROOT_PASSWORD=password

# Copy a SQL script into the container.
COPY myschema.sql /docker-entrypoint-initdb.d