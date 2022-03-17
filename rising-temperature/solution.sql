SELECT w2.id FROM Weather w1, Weather w2 WHERE DATE_ADD(w1.recordDate, INTERVAL 1 DAY)=w2.recordDate AND w2.temperature > w1.temperature;
