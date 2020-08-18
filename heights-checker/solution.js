/**
 * @param {number[]} heights
 * @return {number}
 */
var heightChecker = function(heights) {
  var sorted = [...heights].sort((a, b) => a-b);
  var count = 0;
  for (var i=0;i<heights.length;i++) {
    var h = heights[i];
    var s = sorted[i];
    if (h !== s) {
      count++;
    }
  }
  return count;
};
