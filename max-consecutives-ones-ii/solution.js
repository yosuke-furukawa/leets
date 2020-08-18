/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaxConsecutiveOnes = function(nums) {
  if (nums.length === 0) {
    return 0;
  }
  var i = -1;
  var flipped = 0;
  var max = 1;
  for (var j=0;j<nums.length;j++) {
    var n = nums[j];
    if (flipped === 0 && n === 0) {
      flipped = j+1;
    } else if (n === 0) {
      var len = j-i-1;
      max = Math.max(max, len);
      i = flipped-1;
      j = flipped-1;
      flipped = 0;
    }
  }
  if (j === nums.length) {
    var len = j-i-1;
    max = Math.max(max, len);
  }
  
  return max;
};
