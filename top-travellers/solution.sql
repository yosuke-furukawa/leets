SELECT u.name as name, IFNULL(SUM(r.distance), 0) as travelled_distance FROM Users u LEFT OUTER JOIN Rides r ON u.id = r.user_id GROUP BY u.id ORDER BY travelled_distance DESC, name ASC;
