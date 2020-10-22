var _multiply = function(A, B, x, y) {
  var sum = 0;
  for (var i=0;i<A[x].length;i++) {
    sum += A[x][i] * B[i][y];
  }
  return sum;
}

/**
 * @param {number[][]} A
 * @param {number[][]} B
 * @return {number[][]}
 */
var multiply = function(A, B) {
  var axlen = A.length;
  var aylen = A[0].length;
  var bxlen = B.length;
  var bylen = B[0].length;
  var C = new Array(axlen);
  for (var i=0;i<C.length;i++) {
    C[i] = new Array(bylen).fill(0);
  }
  
  for (var cx=0;cx<C.length;cx++) {
    for (var cy=0;cy<C[cx].length;cy++) {
      C[cx][cy] = _multiply(A, B, cx, cy);
    }
  }
  return C;
};
