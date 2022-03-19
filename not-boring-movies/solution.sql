SELECT id, movie, description, rating FROM Cinema WHERE description NOT LIKE "%boring%" AND MOD(id, 2) = 1 ORDER BY rating DESC;
