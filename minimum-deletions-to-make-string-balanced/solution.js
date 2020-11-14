/**
 * @param {string} s
 * @return {number}
 */
var minimumDeletions = function(s) {
  var n = s.length;
  var A = new Array(n).fill(0);
  A[n - 1] = s[n - 1] === 'a' ? 1 : 0;
  for (var i = n - 2; i >= 0; i--) {
    if (s[i] === "a") {
      A[i] = A[i+1] + 1;
    } else {
      A[i] = A[i+1];
    }
  }
  var result = Infinity;
  var bCount = 0;
  for (var i = 0; i < n; i++) {
    if(s[i] === "b") {
      result = Math.min(result, bCount + A[i]);
      bCount++;
    }
  }

  result = Math.min(result, bCount);
        
  return result == Infinity ? 0 : result;
};
