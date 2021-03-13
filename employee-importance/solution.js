/**
 * Definition for Employee.
 * function Employee(id, importance, subordinates) {
 *     this.id = id;
 *     this.importance = importance;
 *     this.subordinates = subordinates;
 * }
 */

/**
 * @param {Employee[]} employees
 * @param {number} id
 * @return {number}
 */
var GetImportance = function(employees, id) {
  const map = new Map();
  for (const employee of employees) {
    map.set(employee.id, employee);
  }
  
  const queue = [id];
  let answer = 0;
  while (queue.length > 0) {
    const itemId = queue.shift();
    const employee = map.get(itemId);
    if (employee) {
      answer += employee.importance;
      if (employee.subordinates.length > 0) {
        queue.push(...employee.subordinates);
      }
    }
  }
  return answer;
};
