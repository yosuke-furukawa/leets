# Write your MySQL query statement below
SELECT COALESCE(ROUND(e1.accepts/e2.requests,2), 0) as accept_rate FROM
  (SELECT COUNT(DISTINCT requester_id, accepter_id) as accepts FROM request_accepted) e1,
  (SELECT COUNT(DISTINCT sender_id, send_to_id) as requests FROM friend_request) e2;
  
