/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
    var exp = (nums.length+1)*nums.length/2
    var sum = nums.reduce((a, c) => a+c);
    return exp - sum
};
