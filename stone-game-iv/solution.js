/**
 * @param {number} n
 * @return {boolean}
 */
var dp = [false, true, false];
var winnerSquareGame = function(n) {
  if (dp[n] != null) {
    return dp[n];
  }
  for (var i=3;i<=n;i++) {
    var s = parseInt(Math.sqrt(i)) ** 2;
    if (s === i) {
      dp[i] = true;
      continue;
    }
    for (var j=s;j>0;j = (Math.sqrt(j) - 1)**2) {
      if (dp[i-j] === false) {
        dp[i] = !dp[i-j];
        break;
      }
    }
    if (dp[i] === true) {
      continue;
    }
    dp[i] = false;
  }
  
  return dp[n];
};
