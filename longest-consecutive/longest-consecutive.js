/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function(nums) {
  if (nums.length === 0) {
    return 0;
  }
  var map = new Map();
  var max = 0;
  for (var num of nums) {
    if (!map.has(num)) {
      var left = map.has(num-1) ? map.get(num-1) : 0;
      var right = map.has(num+1) ? map.get(num+1) : 0;
    
      max = Math.max(left+right+1, max);
      map.set(num, left+right+1);      
      map.set(num-left, left+right+1);
      map.set(num+right, left+right+1);
    }
  }
  return max;
};
