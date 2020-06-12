/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
    nums = nums.sort((a, b) => a-b);
    for (var i=0;i<=nums.length;i++){
        if (nums[i] !== i) {
            return i;
        }
    }
};
