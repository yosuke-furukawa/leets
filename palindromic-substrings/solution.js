/**
 * @param {string} s
 * @return {number}
 */
var countSubstrings = function(s) {
  var n = s.length;
  var count = 0;
  var dp = new Array(n).fill(false).map(() => new Array(n).fill(false));
  
  for (var i=0;i<n;i++) {
    dp[i][i] = true;
  }
  
  for (var l=2;l<=n;l++) {
    for (var i=0;i<=n-l;i++) {
      var j=i+l-1;
      
      if (l === 2) {
        if (s[i] === s[j]) {
          dp[i][j] = true;
        }
      }
      
      if (s[i] === s[j] && dp[i+1][j-1]) {
        dp[i][j] = true;
      }
    }
  }
  
  for (var i=0;i<dp.length;i++) {
    for (var j=0;j<dp[i].length;j++) {
      if (dp[i][j]) {
        count++;
      }
    }
  }
  return count;
};
