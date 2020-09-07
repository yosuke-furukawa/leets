/**
 * @param {string} pattern
 * @param {string} str
 * @return {boolean}
 */
var wordPattern = function(pattern, str) {
  var patterns = pattern.split("");
  var strs = str.split(" ");
  var map1 = new Map();
  var map2 = new Map();
  if (patterns.length !== strs.length) {
    return false;
  }
  for (var i=0;i<patterns.length;i++) {
    var p = patterns[i];
    var s = strs[i];
    if (map1.has(p)) {
      if (map1.get(p) !== s) {
        return false;
      }
    }
    if (map2.has(s)) {
      if (map2.get(s) !== p) {
        return false;
      }
    }
    map1.set(p, s);
    map2.set(s, p);
  }
  return true;
};
