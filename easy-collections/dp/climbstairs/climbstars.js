var climbStairs = function(n) {
  var memo = [1, 2, 3];
  for (var i=2; i<=n; i++) {
    memo[i] = memo[i-1] + memo[i-2];
  }
  return memo[n];
};
