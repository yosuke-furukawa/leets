SELECT DISTINCT l1.num as ConsecutiveNums FROM Logs l1 INNER JOIN Logs l2 ON l1.id = l2.id - 1 INNER JOIN Logs l3 ON l2.id = l3.id - 1 WHERE l1.num = l2.num AND l2.num = l3.num;
