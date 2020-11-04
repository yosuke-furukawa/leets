
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findMinHeightTrees = function(n, edges) {
  var matrix = new Array(n);
  for (var i=0;i<n;i++) {
    matrix[i] = new Array(n).fill(0);
  }
  
  for (var edge of edges) {
    matrix[edge[0]][edge[1]] = 1;
    matrix[edge[1]][edge[0]] = 1;
  }
  // console.log(matrix);
  var heights = new Array(n).fill(Infinity);
  var min = Infinity;
  for (var i=0;i<n;i++) {
    var height = 0;
    var queue = [[i, height]];
    var set = new Set([i]);
    while (queue.length > 0) {
      var [node, h] = queue.shift();
      height = Math.max(height, h);
      if (height > min) {
        break;
      }
      var m = matrix[node].map((v, i) => (v === 1) ? i : -1).filter((x) => x !== -1).filter((v) => !set.has(v));
      // console.log({m, set, height, queue});
      if (m.length > 0) {
        for (var q of m) {
          set.add(q);
          queue.push([q, h+1]);
        }
      }
    }
    min = Math.min(min, height);
    heights[i] = height;
  }
  // console.log(heights);
  var min = Math.min(...heights);
  return heights.map((x, i) => [x,i]).filter(([x]) => x===min).map(([x, i]) => i);
};
