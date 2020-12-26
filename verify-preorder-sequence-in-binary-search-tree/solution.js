/**
 * @param {number[]} preorder
 * @return {boolean}
 */
var verifyPreorder = function(preorder) {
  var low = -Infinity;
  var stack = [];
  
  for (let p of preorder) {
    if (p < low) {
      return false;
    }
    while(stack.length > 0 && p > stack[stack.length-1]) {
      low = stack.pop();
    }
    stack.push(p);
  }
  return true;
};
