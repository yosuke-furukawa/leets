# Write your MySQL query statement below
SELECT 
  Score,
  dense_rank() over(ORDER BY Score DESC) as 'Rank'
FROM Scores;
