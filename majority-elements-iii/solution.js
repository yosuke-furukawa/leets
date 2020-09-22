/**
 * @param {number[]} nums
 * @return {number[]}
 */
var majorityElement = function(nums) {
  if (nums.length === 1) {
    return nums;
  }
  
  if (nums.length === 2) {
    if (nums[0] !== nums[1]) {
      return nums;
    }
    
    return [nums[0]];
  }
  var thresh = nums.length / 3;
  var sorted = nums.sort((a, b) => a-b);
  var count = 1;
  var result = [];
  for (var i=1;i<sorted.length;i++) {
    var s1 = sorted[i-1];
    var s2 = sorted[i];
    if (s1 === s2) {
      count++;
    }
    if (s1 !== s2) {
      count = 1;
    }
    if (count > thresh && result[result.length-1] !== s2) {
      result.push(s2);
    }
  }
  return result;
};
