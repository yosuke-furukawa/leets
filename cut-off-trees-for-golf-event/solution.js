/**
 * @param {number[][]} forest
 * @return {number}
 */
var dr = [-1, 1, 0, 0];
var dc = [0, 0, -1, 1];

var dist = function(forest, sr, sc, tr, tc) {
  var R = forest.length;
  var C = forest[0].length;
  
  var processed = new Set();
  var deque = [];
  deque.unshift([0, sr, sc]);
  while (deque.length > 0) {
    var cur = deque.shift();
    var detours = cur[0];
    var r = cur[1];
    var c = cur[2];
    
    if (!processed.has(r*C + c)) {
      processed.add(r*C + c);
      if (r === tr && c === tc) {
        return Math.abs(sr-tr) + Math.abs(sc-tc) + 2 * detours;
      }
      
      for (var di=0;di<4;++di) {
        var nr = r + dr[di];
        var nc = c + dc[di];
        
        var closer = false;
        if (di <= 1) {
          closer = di === 0 ? r > tr : r < tr;
        } else {
          closer = di === 2 ? c > tc : c < tc;
        }
        if (0 <= nr && nr < R && 0 <= nc && nc < C && forest[nr][nc] > 0) {
          if (closer) {
            deque.unshift([detours, nr, nc]);
          } else {
            deque.push([detours+1, nr, nc]);
          }
        }
      }
    }
  }
  return -1;
}
  
var cutOffTree = function(forest) {
  var trees = [];
  
  for (var r = 0; r < forest.length; ++r) {
    for (var c = 0; c < forest[r].length; ++c) {
      var v = forest[r][c];
      if (v > 1) trees.push([v, r, c]);
    }
  }
  trees = trees.sort((a, b) => a[0] - b[0]);
  
  var ans = 0, sr = 0, sc = 0;
  for (var tree of trees) {
    var d = dist(forest, sr, sc, tree[1], tree[2]);
    if (d < 0) return -1;
    ans += d;
    sr = tree[1]; sc = tree[2];
  }
  return ans;
};
