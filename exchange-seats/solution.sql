SELECT
IF(id < (SELECT COUNT(*) FROM seat), IF(id MOD 2=0, id-1, id+1), IF(id MOD 2=0, id-1, id)) as id, student
FROM seat
ORDER BY id ASC;
