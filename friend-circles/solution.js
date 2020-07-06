/**
 * @param {number[][]} M
 * @return {number}
 */
function find(parent, i) {
  if (parent[i] == -1) {
    return i;
  }
  return find(parent, parent[i]);
}

function union(parent, x, y) {
  var xset = find(parent, x);
  var yset = find(parent, y);
  if (xset !== yset) {
    parent[xset] = yset;
  }
}

var findCircleNum = function(M) {
  var parent = new Array(M.length).fill(-1);
  
  for (var i = 0; i < M.length; i++) {
    for (var j = 0; j < M.length; j++) {
      if (M[i][j] === 1 && i != j) {
        union(parent, i, j);
      }
    }
  }
  var count = 0;
  for (var i = 0; i < parent.length; i++) {
    if (parent[i] == -1) count++;
  }
  return count;
};
