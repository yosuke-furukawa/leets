/**
 * @param {string} s
 * @param {string} t
 * @return {character}
 */
var findTheDifference = function(s, t) {
  var map = new Map();
  
  for (var cs of s) {
    if (map.has(cs)) {
      map.set(cs, map.get(cs)+1);
    } else {
      map.set(cs, 1);
    }
  }
  
  for (var ct of t) {
    if (!map.has(ct)) {
      return ct;
    } else {
      var val = map.get(ct);
      if (val - 1 === 0) {
        map.delete(ct);
      } else {
        map.set(ct, val-1);
      }
    }
  }
};
