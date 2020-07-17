/**
 * @param {number[][]} matrix
 * @param {number} k
 * @return {number}
 */
var kthSmallest = function(matrix, k) {
  var minIndex = 0;
  var min = Infinity;
  var i=0;
  while (matrix.length > 0) {
    var col = i % matrix.length;
    var c = matrix[col][0];
    if (min > c) {
      min = c;
      minIndex = col;
    }
    
    if (col === matrix.length-1) {
      k--;
      if (k<=0) {
        return min;
      }
      matrix[minIndex].shift();
      if (matrix[minIndex].length === 0) {
        matrix.splice(minIndex, 1);
        i=0;
        min = Infinity;
        continue;
      }
      min = Infinity;
    }
    i++;
  }
};
