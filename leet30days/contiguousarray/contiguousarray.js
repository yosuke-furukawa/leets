/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaxLength = function(nums) {
    var map = new Map();
    map.set(0, -1);
    var count = 0;
    var maxlen = 0;
    for (var i=0;i<nums.length;i++) {
        if (nums[i] === 0) {
            count--;
        } else {
            count++;
        }
        
        if (map.has(count)) {
            maxlen = Math.max(maxlen, i - map.get(count));
        } else {
            map.set(count, i);
        }
    }
    return maxlen;
};
