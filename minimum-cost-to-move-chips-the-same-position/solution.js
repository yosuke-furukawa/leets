/**
 * @param {number[]} position
 * @return {number}
 */
var minCostToMoveChips = function(position) {
  var oddCount = 0;
  var evenCount = 0;
  for (var p of position) {
    if (p%2 === 0) {
      evenCount++;
    } else {
      oddCount++;
    }
  }
  return Math.min(oddCount, evenCount);
};
