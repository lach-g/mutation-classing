# container name = mutants-db-container
# database name = testdb
docker exec mutants-db-container /usr/bin/mysqldump -u root --password=password testdb > backup.sql
