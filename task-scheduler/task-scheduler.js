/**
 * @param {character[]} tasks
 * @param {number} n
 * @return {number}
 */
var leastInterval = function(tasks, n) {
  var taskmap = new Map();
  var priorityqueue = [];
  var completed = [];
  for (const task of tasks) {
    var count = taskmap.get(task) || 0;
    count++;
    taskmap.set(task, count);
  }

  priorityqueue = [...taskmap.entries()].map(
    ([task, priority]) => ({ task, priority })).sort(
    (a, b) => b.priority - a.priority);
  
  while(priorityqueue.length > 0) {
    var task = priorityqueue.shift();
    completed.push(task.task);
    task.priority--;
    if (task.priority > 0) {
      while (priorityqueue.length < n) {
        priorityqueue.push({task: "(idle)", priority:-1 });
      }
      var size = priorityqueue.length;
      for (var i=n; i<priorityqueue.length;i++) {
        if (priorityqueue[i].priority <= task.priority) {
          priorityqueue.splice(i, 0, task);
          break;
        }
      }
      if (size === priorityqueue.length) {
        priorityqueue.push(task);
      }
    }
  }
  return completed.length;
};
