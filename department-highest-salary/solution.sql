SELECT d.name as Department, e.name as Employee, e.salary as Salary FROM (
  SELECT e1.id, e1.name, e1.salary, e1.departmentId FROM Employee e1 LEFT OUTER JOIN (
    SELECT departmentId, MAX(salary) as max FROM Employee e GROUP BY e.departmentId
  ) e2 ON e1.departmentId = e2.departmentId WHERE salary = max
) e INNER JOIN Department d ON e.departmentId = d.id;
