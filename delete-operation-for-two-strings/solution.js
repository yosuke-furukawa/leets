/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function(word1, word2) {
  var len1 = word1.length; 
  var len2 = word2.length;

  let dp = [];
  for (var i=0;i<=len1;i++) {
    dp[i] = [];
    dp[i][0] = 0;
  }
  
  for (var i=0;i<=len2;i++) {
    dp[0][i] = 0;
  }
  
  
  for (var i=1;i<=len1;i++) {
    for (var j=1;j<=len2;j++) {
      if (word1[i-1] === word2[j-1]) {
        dp[i][j] = dp[i-1][j-1] + 1;
      } else {
        dp[i][j] = Math.max(dp[i][j-1], dp[i-1][j]);
      }
    }
  }
  return len1 + len2 - 2 * dp[len1][len2];
};
