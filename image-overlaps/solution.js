var shiftAndCount = (xShift, yShift, M, R) => {
  var count = 0;
  var rRow = 0;
  for (var mRow = yShift; mRow < M.length; mRow++) {
    var rCol = 0;
    for (var mCol = xShift; mCol < M.length; mCol++) {
      if (M[mRow][mCol] === 1 && M[mRow][mCol] == R[rRow][rCol]) {
        count++;
      }
      rCol++;
    }
    rRow++;
  }
  return count;
};

/**
 * @param {number[][]} A
 * @param {number[][]} B
 * @return {number}
 */
var largestOverlap = function(A, B) {
  var maxOverlaps = 0;
  for (var yShift=0;yShift<A.length;yShift++) {
    for (var xShift=0;xShift<A.length;xShift++) {
      maxOverlaps = Math.max(maxOverlaps, shiftAndCount(xShift, yShift, A, B));
      maxOverlaps = Math.max(maxOverlaps, shiftAndCount(xShift, yShift, B, A));
    }
  }
  return maxOverlaps;
};
