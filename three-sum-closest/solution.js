
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var threeSumClosest = function(nums, target) {
  nums.sort((a,b) =>  a-b);
  var min = Infinity;
  var result = 0;
  for(var i = 0; i < nums.length; i++) {
    var front = i+1;
    var end = nums.length-1;
    
    while(front < end) {
      const sum = nums[i] + nums[front] + nums[end];
      const diff = Math.abs(target - sum);
      if (min > diff) {
        min = diff;
        result = sum;
      }
      if(sum > target) {
        end--;
      } else {
        front++;
      }
    }
  }
  return result;
};
