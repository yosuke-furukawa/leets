/**
 * @param {number[]} A
 * @return {number[]}
 */
var sortArrayByParity = function(A) {
  for (var i=0;i<A.length;i++) {
    var a = A[i];
    if (a < 0) {
      A[i] = -A[i];
    } else if (a % 2 === 1) {
      A.push(...A.splice(i, 1).map((a) => -a));
      i--;
    }
  }
  return A;
};
