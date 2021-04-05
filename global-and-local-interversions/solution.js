/**
 * @param {number[]} A
 * @return {boolean}
 */
var isIdealPermutation = function(A) {
  for (var i=0;i<A.length;i++) {
    let a = A[i];
    if (Math.abs((a - i)) > 1) {
      return false;
    }
  }
  return true;
};
