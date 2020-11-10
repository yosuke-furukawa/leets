/**
 * @param {number[][]} A
 * @return {number[][]}
 */
var flipAndInvertImage = function(A) {
  var B = new Array(A.length);
  for (var x=0;x<A.length;x++) {
    B[x] = [];
    for (var y=A[x].length-1,z=0;y>=0;y--,z++) {
      B[x][z] = Math.abs(A[x][y]-1);
    }
  }
  return B;
};
