/**
 * @param {number[]} heights
 * @return {number}
 */
var largestRectangleArea = function(heights) {
  var stack = [-1];
  var maxArea = 0;
  for (var i=0;i<heights.length;i++) {
    while(stack[stack.length-1] !== -1 && heights[stack[stack.length-1]] >= heights[i]) {
      var lastElementIndex = stack.pop();
      maxArea = Math.max(maxArea, heights[lastElementIndex] * (i - stack[stack.length-1] - 1));
    }
    stack.push(i);
  }
  while(stack[stack.length-1] !== -1) {
    var lastElementIndex = stack.pop()
    maxArea = Math.max(maxArea, heights[lastElementIndex] * (heights.length - stack[stack.length-1] - 1))
  }
  return maxArea;
};
