/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var subarraySum = function(nums, k) {
    var sumsmap = new Map();
    var sum = 0;
    var count = 0;
    sumsmap.set(0, 1);
    for (var i=0; i<nums.length; i++) {
        sum += nums[i];
        if (sumsmap.has(sum - k)) {
            count += sumsmap.get(sum - k);
        }
        sumsmap.set(sum, (sumsmap.get(sum) || 0) + 1);
    }
    return count;
};
