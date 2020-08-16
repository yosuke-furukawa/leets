/**
 * @param {number[]} A
 * @return {number[]}
 */
var sortedSquares = function(A) {
  var B = A.map((a) => a * a);
  B = B.sort((a, b) => a-b);
  return B;
};
