# container name = mutants-db-container
# database name = testdb
cat backup.sql | docker exec -i mutants-db-container /usr/bin/mysql -u root --password=password testdb
