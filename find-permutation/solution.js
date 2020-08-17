
/**
 * @param {string} s
 * @return {number[]}
 */
var findPermutation = function(s) {
  var ps = [];
  for (var i=2;i<=s.length+1;i++) {
    ps.push(i);
  }
  var result = [1];
  var position = 0;
  var i = 0;
  while (ps.length > 0) {
    var c = s[i];
    var n = ps.shift();
    if (c === "D") {
      if (position === 0) {
        result.unshift(n);
      } else {
        result.splice(position-1, 0, n);
      }
    } else {
      position = result.length;
      result.splice(position + 1, 0, n);
      position++;
    }
    i++;
  }
  return result;
};


