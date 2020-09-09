/**
 * @param {number[][]} graph
 * @return {boolean}
 */
var isBipartite = function(graph) {
  var colors = new Array(graph.length).fill(0);
  
  var helper = (ci, c) => {
    if (colors[ci] === c) {
      return true;
    }
    if (colors[ci] === -c) {
      return false;
    }
    
    colors[ci] = c;
    var neighbors = graph[ci];
    var result = true;
    for (var neighbor of neighbors) {
      result = result && helper(neighbor, -c);
    }
    return result;
  };
   
  var result = helper(0, 1);
  if (result) {
    for (var i =0;i<colors.length;i++) {
      if (colors[i] === 0) {
        var r = helper(i, 1);
        result = result && r;
      }
    }
  }
  return result;
};
