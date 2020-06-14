/**
 * @param {number[]} nums
 * @return {number[]}
 */
var productExceptSelf = function(nums) {
    var answer = [1];
    for (var i=1;i<nums.length;i++) {
        answer[i] = nums[i-1] * answer[i-1];
    }

    var R = 1;
    for (var i=answer.length-1; i>=0; i--) {
        answer[i] = answer[i] * R;
        R = R * nums[i];
    }
    return answer;
};
