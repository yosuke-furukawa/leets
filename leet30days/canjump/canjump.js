/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canJump = function(nums) {
    var goal = nums.length - 1;
    var pos  = goal
    if (goal === 0) return true;
    for(var i=goal-1;i>=0; i--) {
        var to = i + nums[i];
        if (to >= goal) {
            pos = i;
        } else if (to >= pos) {
            pos = i;
        }
    }
    return pos === 0;
};
