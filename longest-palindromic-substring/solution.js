/**
 * @param {string} s
 * @return {string}
 */
var longestPalindrome = function(s) {
  var tables = new Array(n);
  var n = s.length;
  var start = 0;
  var max = 1;
  for (var i=0;i<n;i++) {
    tables[i] = new Array(n).fill(false);
    tables[i][i] = true;
  }

  for (var i=0;i<n-1;i++) {
    if (s[i] === s[i+1]) {
      tables[i][i+1] = true;
      start = i;
      max = 2;
    } 
  }

  for (var k=3;k<=n;k++) {
    for (var i=0;i<n-k+1;i++) {
      var j = i + k - 1;
      if (tables[i + 1][j - 1] && s[i] === s[j]) { 
        tables[i][j] = true; 

        if (k > max) { 
          start = i; 
          max = k; 
        } 
      } 
    }
  }
  var end = start + max;

  return s.substring(start, end);
};
