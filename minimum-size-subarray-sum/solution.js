/**
 * @param {number} s
 * @param {number[]} nums
 * @return {number}
 */
var minSubArrayLen = function(s, nums) {
  var min = Number.MAX_VALUE;
    
    // boundaries
    var l = 0;
    var r = -1;
    
    // current sum
    var sum = 0;
    
    while (r < nums.length) {
        if (sum >= s) {
            min = Math.min(min, r - l + 1);
            sum -= nums[l];
            l++;
        } else {
            r++;
            sum += nums[r];
        }
    }
    
    return min === Number.MAX_VALUE ? 0 : min;
};
