/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function(nums) {
    var len = nums.length;
    if (len === 0) return 0;
    if (len === 1) return nums[0];
    
    var memo = [nums[0], Math.max(nums[0], nums[1])];
    for (var i=2; i<len;i++){
        memo[i] = Math.max(memo[i-1], memo[i-2] + nums[i]);
    }
    return memo[len-1]
};
