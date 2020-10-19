var countRotates = (A, B, a0, begin = 0) => {
  var result = begin;
  for (var i=1;i<A.length;i++) {
    var a1 = A[i];
    var b1 = B[i];
    if (a1 !== a0 && b1 === a0) {
      result++;
      continue;
    } else if (a1 !== a0) {
      return -1;
    }
  }
  return result;
};

/**
 * @param {number[]} A
 * @param {number[]} B
 * @return {number}
 */
var minDominoRotations = function(A, B) {
  var rotate1 = countRotates(A, B, A[0]);
  var rotate2 = countRotates(B, A, B[0]);
  var rotate3 = countRotates(A, B, B[0], 1);
  var rotate4 = countRotates(B, A, A[0], 1);
  var answers = [rotate1, rotate2, rotate3, rotate4].filter((r) => r >= 0);
  return answers.length > 0 ? Math.min(...answers) : -1;
};
