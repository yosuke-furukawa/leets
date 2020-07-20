/**
 * @param {string} s
 * @return {number}
 */
var numDecodings = function(s) {
  var dp = new Array(s.length).fill(0);
  if (s[0] != '0') {
    dp[0] = 1;
  }
  for (var i=1;i<s.length;i++) {
    var x = +s[i];
    var y = +s.substring(i-1, i+1);
    if (x >= 1 && x <= 9) {
        dp[i] += dp[i-1];
    }
    if (y >= 10 && y<= 26) {
      if (i>=2) {
        dp[i] += dp[i-2];
      } else {
        dp[i] += 1;
      }
    }
  }

  return dp[s.length-1];
};
