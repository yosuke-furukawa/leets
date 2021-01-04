/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {boolean}
 */
var validTree = function(n, edges) {
  if (edges.length === 0 && n == 1) {
    return true;
  }
  var graph = Array(n).fill(0).map(() => Array(n).fill(0));
  for (var edge of edges) {
    graph[edge[0]][edge[1]] = 1;
    graph[edge[1]][edge[0]] = 1;
  }
  if (edges.length === 0 ) {
    return false;
  }
  // console.log(graph);
  var isClosed = false;
  var dfs = (edge, path, prev) => {
    // console.log({edge, path, prev});
    if (isClosed) {
      return;
    }
    for (var i=0;i<graph[edge].length;i++) {
      if (i === prev) {
        continue;
      }
      var p = graph[edge][i];
      if (p === 0) {
        continue;
      }
      if (path.has(i)) {
        isClosed = true;
        return;
      } else {
        path.add(i);
        dfs(i, path, edge);
      }
    }
  }
  var set = new Set();
  set.add(edges[0][0]);
  dfs(edges[0][0], set);
  // console.log(set);
  if (isClosed) {
    return false;
  }
  if (set.size !== n) {
    return false;
  }
  return true;
};
