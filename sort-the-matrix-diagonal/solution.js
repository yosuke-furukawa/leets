/**
 * @param {number[][]} mat
 * @return {number[][]}
 */
var diagonalSort = function(mat) {
  var result = Array(mat.length).fill(0).map(() => Array(mat[0].length));

  for (var col=0;col<mat[0].length;col++) {
    var arr = [];
    for (var i=0;;i++) {
      var val = mat[0+i]?.[col+i];
      if (val == null) {
        break;
      }
      arr.push(val);
    }
    arr.sort((a, b) => a-b);
    for (var i=0;;i++) {
      var val = mat[0+i]?.[col+i];
      if (val == null) {
        break;
      }
      mat[0+i][col+i] = arr[i];
    }    
  }
  
  for (var row=1;row<mat.length;row++) {
    var arr = [];
    for (var i=0;;i++) {
      var val = mat[row+i]?.[0+i];
      if (val == null) {
        break;
      }
      arr.push(val);
    }
    arr.sort((a, b) => a-b);
    for (var i=0;;i++) {
      var val = mat[row+i]?.[0+i];
      if (val == null) {
        break;
      }
      mat[row+i][0+i] = arr[i];
    }
  }
  return mat;
};
