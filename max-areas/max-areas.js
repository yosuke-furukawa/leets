/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function(height) {
  var start = 0;
  var last = height.length-1;
  var max = 0;
  while(start < last) {
    var h = Math.min(height[start], height[last]);
    max = Math.max(max, h * (last - start));
    if (height[start] < height[last]) {
      start++;
    } else if (height[start] > height[last]) {
      last--;
    } else {
      var mid = start + Math.floor((last - start)/2);
      if ((mid - start) < (last - mid)) {
        start++;
      } else {
        last--;
      }
    }
  }
  return max;
};
