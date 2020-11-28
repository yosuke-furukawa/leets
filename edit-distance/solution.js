/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function(word1, word2) {
  var dp = new Array(word1.length+1).fill(0).map(() => new Array(word2.length+1).fill(0));
  
  for (var i=0;i<word1.length+1;i++) {
    dp[i][0] = i;
  }
  
  for (var j=0;j<word2.length+1;j++) {
    dp[0][j] = j;
  }
  
  for (var i=0;i<word1.length;i++) {
    for (var j=0;j<word2.length;j++) {
      var cost = word1[i] == word2[j] ? 0 : 1;
      dp[i+1][j+1] = Math.min(dp[i][j+1]+1, dp[i+1][j]+1, dp[i][j] + cost);
    }
  }
  // console.log(dp);
  return dp[word1.length][word2.length];
};
