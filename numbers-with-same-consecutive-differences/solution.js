/**
 * @param {number} N
 * @param {number} K
 * @return {number[]}
 */
var dp = new Array(9);
dp[0] = [];
for (var i=0;i<=9;i++) {
  dp[0][i] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
}
var numsSameConsecDiff = function(N, K) {
  if (dp[N-1]?.[K] != null) {
    return dp[N-1][K];
  }
  for (var n=2;n<=N;n++) {
    for (var k=0;k<=K;k++) {
      if (dp[n-1] != null && dp[n-1][k] != null) {
        continue;
      }
      var prev = [...dp[n-2][k]];
      var next = [];
      for (var i=0;i<10;i++) {
        for (var p of prev) {
          var ps = p+"";
          if (k === Math.abs(+ps[ps.length-1]-i)) {
            var s = ps+""+i;
            if (s[0] === "0") {
              continue;
            }
            next.push(+s);
          }
        }
      }
      if (dp[n-1] == null) {
        dp[n-1] = [];
      }
      dp[n-1][k] = next;
    }
  }
  if (dp[N-1]?.[K] != null) {
    return dp[N-1][K];
  }
};
