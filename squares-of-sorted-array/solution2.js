/**
 * @param {number[]} A
 * @return {number[]}
 */
var sortedSquares = function(A) {
  var N = A.length;
  var j = 0;
  while (j<N && A[j] < 0) {
    j++;
  }
  var i = j-1;
  var ans = new Array(N);
  var t = 0;
  while (i >= 0 && j < N) {
    if (A[i] * A[i] < A[j] * A[j]) {
      ans[t++] = A[i] * A[i];
      i--;
    } else {
      ans[t++] = A[j] * A[j];
      j++;
    }
  }
  while (i >= 0) {
    ans[t++] = A[i] * A[i];
    i--;
  }
  while (j < N) {
    ans[t++] = A[j] * A[j];
    j++;
  }

  return ans;
};
