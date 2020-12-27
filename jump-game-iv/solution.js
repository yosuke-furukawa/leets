
/**
 * @param {number[]} arr
 * @return {number}
 */
var minJumps = function(arr) {
  var map = new Map();
  for (var i=0;i<arr.length;i++) {
    var num = arr[i];
    if (map.has(num)) {
      map.get(num).add(i);    
    } else {
      var node = new Set();
      node.add(i);
      map.set(num, node);
    }
  }
  
  var graph = {};
  for(var index=0;index<arr.length;index++) {
    var nodes = [];
    if (index >= 1) {
      nodes.push(index-1);
    }
    if (index + 1 < arr.length) {
      nodes.push(index+1);
    }
    
    graph[index] = nodes;
  }

  var visited = new Set();
  var numVisited = new Set();

  function bfs(position) {
    var queue = [];
    visited.add(position);
    queue.push([position, 0]);
    while (queue.length > 0) {
      var [pos, distance] = queue.shift();
      if (pos === arr.length - 1) {
        return distance;
      }
      
      for (var p of graph[pos]) {
        if (visited.has(p)) {
          continue;
        }
        
        visited.add(p);
        queue.push([p, distance+1]);
      }
      var val = map.get(arr[pos]);
      if (numVisited.has(val)) {
        continue;
      }
      numVisited.add(val);
      for (var r of val) {
        if (visited.has(r)) {
          continue;
        }
        visited.add(r);
        numVisited.add(val);
        queue.push([r, distance+1]);
      }
      
    }
  }
  var diff = bfs(0);
  return diff;
};

