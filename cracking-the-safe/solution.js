/**
 * @param {number} n
 * @param {number} k
 * @return {string}
 */
var crackSafe = function(n, k) {
  var M = Math.pow(k, n-1);
  var P = new Array(M * k);
  for (var i = 0; i < k; ++i)
    for (var q = 0; q < M; ++q)
      P[i*M + q] = q*k + i;

  var ans = "";
  for (var i = 0; i < M*k; ++i) {
    var j = i;
    while (P[j] >= 0) {
      ans += "" + (j / M)|0;
      var v = P[j];
      P[j] = -1;
      j = v;
    }
  }

  for (var i = 0; i < n-1; ++i)
    ans += "0";
  return ans;
};
