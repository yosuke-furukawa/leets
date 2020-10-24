/**
 * @param {string} s
 * @return {number[][]}
 */
var largeGroupPositions = function(s) {
  var results = [];
  for (var i=0;i<s.length;i++) {
    var r = [i];
    for (var j=i+1;j<s.length+1;j++) {
      if (s[i] !== s[j]) {
        if (j - i >= 3) {
          r.push(j-1);
          results.push(r);
          i = j-1;
        }
        break;
      }
    }
  }
  return results;
};
