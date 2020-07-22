/**
 * @param {number} n
 * @return {number}
 */
var numSquares = function(n) {
  var a = Math.sqrt(n);
  if (Number.isInteger(a)) {
    return 1;
  }
  
  var num = 0;
  var nums = new Array(Math.floor(a)).fill(0);
  nums = nums.map((_, i) => (i+1) ** 2);
  var dp = [1];

  for (var i=1; i<=n; i++) {
    var a = Math.sqrt(i);
    if (Number.isInteger(a)) {
      dp[i] = 1;
    } else {
      var re = nums.filter((n) => n < i);
      
      var min = dp[i-1] + 1;
      for (var r of re) {
        var tmp = dp[i-r]+1;
        if (min > tmp) {
          min = tmp;
        }
      }
      dp[i] = min;    
    }
  } 
  return dp[n];
};
