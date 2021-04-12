/**
 * @param {number} n
 * @param {number} k
 * @return {number[]}
 */
var constructArray = function(n, k) {
  let ans = Array(n).fill(0);
  let c = 0;
  for (var v=1;v<n-k;v++) {
    ans[c++] = v;
  }
  for (var i=0;i<=k;i++) {
    ans[c++] = (i % 2 === 0) ? (n - k + i / 2) : (n - i / 2);
  }
  return ans;
};
