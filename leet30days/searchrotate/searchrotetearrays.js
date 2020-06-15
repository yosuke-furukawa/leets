
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function(nums, target) {
    if (nums.length === 0) return -1;
    var lo = 0;
    var hi = nums.length-1;
    while (lo <= hi) {
        var mi = Math.floor((hi + lo)/2);
        if (nums[mi] === target) {
            return mi;
        }
        
        if (nums[lo] <= nums[mi]) {
            if (nums[lo] <= target && target < nums[mi]) {
                hi = mi - 1;
            } else {
                lo = mi + 1;
            }
        } else {
            if (nums[mi] < target && target <= nums[hi]) {
                lo = mi + 1;
            } else {
                hi = mi - 1;
            }
            
        }
    }
    return -1;
};
