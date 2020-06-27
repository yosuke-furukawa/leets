/**
 * @param {character[]} tasks
 * @param {number} n
 * @return {number}
 */
var leastInterval = function(tasks, n) {
  var taskmap = new Map();
  for (const task of tasks) {
    var count = taskmap.get(task) || 0;
    count++;
    taskmap.set(task, count);
  }
  console.log(taskmap);

  var fmax = 0;
  for (const entry of taskmap.entries()) {
    fmax = Math.max(fmax, entry[1]);
  }
  console.log(fmax);
  var nmax = 0;
  for (const entry of taskmap.entries()) {
    if (fmax === entry[1]) {
      nmax++;
    }
  }
  console.log(nmax);
  return Math.max(tasks.length, (fmax-1) * (n+1) + nmax);
};
