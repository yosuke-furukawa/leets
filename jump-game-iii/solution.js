/**
 * @param {number[]} arr
 * @param {number} start
 * @return {boolean}
 */
var canReach = function(arr, start) {
  var set = new Set();
  var bfs = (p) => {
    if (arr[p] === 0) {
      return true;
    }
    if (set.has(p)) {
      return false;
    }
    set.add(p);
    
    var queue = [];
    var p1 = p + arr[p];
    if (p1 >= 0 && p1 <= arr.length-1) {
      queue.push(p1);
    }
    var p2 = p - arr[p];
    if (p2 >= 0 && p2 <= arr.length-1) {
      queue.push(p2);
    }
    
    for (var pos of queue) {
      var r = bfs(pos);
      if (r === true) {
        return r;
      }
    }
  }
  return bfs(start) ?? false;
};
