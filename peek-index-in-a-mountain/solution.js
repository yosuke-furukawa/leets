/**
 * @param {number[]} A
 * @return {number}
 */
var peakIndexInMountainArray = function(A) {
  var peek = Math.max(...A);
  var i = A.indexOf(peek);
  return i;
};
