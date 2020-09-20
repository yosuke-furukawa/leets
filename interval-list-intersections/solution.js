/**
 * @param {number[][]} A
 * @param {number[][]} B
 * @return {number[][]}
 */
var intervalIntersection = function(A, B) {
  var C = [];
  var ai = 0;
  var bi = 0;
  while(ai < A.length && bi < B.length) {
    var a = A[ai];
    var b = B[bi];
    
    if (a[0] <= b[1] && a[1] >= b[0]) {
      C.push([Math.max(a[0], b[0]), Math.min(a[1], b[1])]);
    }
    
    if (a[1] < b[1]) {
      ai++;
    } else {
      bi++;
    }
  }
  return C;
};
