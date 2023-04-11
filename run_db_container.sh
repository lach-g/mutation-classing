#!/bin/bash

# If not already created:
# docker run --name mutants-db-container -p 3306:3306 -e MYSQL_ROOT_PASSWORD=password -d mysql

# If already exists:
docker start mutants-db-container 
