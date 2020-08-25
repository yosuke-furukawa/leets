/**
 * @param {number[]} nums
 * @return {number[]}
 */
var sortArray = function(nums) {
  if (nums.length === 1) {
    return nums;
  }
  var pivot = Math.floor(nums.length / 2);
  var left = sortArray([...nums].slice(0, pivot));
  var right = sortArray([...nums].slice(pivot));
  
  return merge(left, right);
};

var merge = function(left, right) {
  var array = [];
  while (left.length > 0 || right.length > 0) {
    var l = left.shift();
    var r = right.shift();
    if (l == null) {
      array.push(r);
      continue;
    }
    if (r == null) {
      array.push(l);
      continue;
    }
    
    if (l < r) {
      array.push(l);
      right.unshift(r);
    } else {
      array.push(r);
      left.unshift(l);
    }
  }
  return array;
}
