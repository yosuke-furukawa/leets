/**
 * @param {number[][]} graph
 * @return {number[][]}
 */

var allPathsSourceTarget = function(graph) {
  var result = [];
  var helper = function(node, target, temp) {
    temp.push(node);
    if (node === target) {
      result.push(temp);
      return;
    }
    for (var i=0;i<graph[node].length;i++) {
      helper(graph[node][i], target, [...temp]);     
    }
  }
  
  helper(0, graph.length-1, []);
  return result;
};
