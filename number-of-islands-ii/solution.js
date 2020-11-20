/**
 * @param {number} m
 * @param {number} n
 * @param {number[][]} positions
 * @return {number[]}
 */
var numIslands2 = function(m, n, positions) {
  if (positions.length === 0) {
    return [];
  }
  var map = new Array(m);
  for (var i=0;i<map.length;i++) {
    map[i] = new Array(n).fill(0);
  }
  
  var [f1, f2] = positions.shift();
  map[f1][f2] = 1;
  var islands = 1;
  var count = 1;
  var result = [islands];
  for (var [x, y] of positions) {
    count++;
    if (map[x][y] > 0) {
      result.push(islands);
      continue;
    }
    var north = map[x-1]?.[y];
    var east = map[x]?.[y+1];
    var west = map[x]?.[y-1];
    var south = map[x+1]?.[y];
    if (north >= 1 || east >= 1 || west >= 1 || south >= 1) {
      var t = [north, east, west, south].filter(Boolean);
      var temp = Math.min(...t);
      var len = Array.from(new Set(t)).length;
      islands -= (len - 1);
      map[x][y] = temp;
      var queue = [[x, y]];
      while (queue.length > 0) {
        let [x, y] = queue.shift();
        let north = map[x-1]?.[y];
        let east = map[x]?.[y+1];
        let west = map[x]?.[y-1];
        let south = map[x+1]?.[y];
        if (north > 0 && north != temp) {
          map[x-1][y] = temp;
          queue.push([x-1,y]);
        }
        if (east > 0 && east != temp) {
          map[x][y+1] = temp;
          queue.push([x,y+1]);
        }
        if (west > 0 && west != temp) {
          map[x][y-1] = temp;
          queue.push([x,y-1]);
        }
        if (south > 0 && south != temp) {
          map[x+1][y] = temp;
          queue.push([x+1,y]);
        }
      }
      result.push(islands);
    } else {
      islands++;
      result.push(islands);
      map[x][y] = count;
    }
  }
  return result;
};
