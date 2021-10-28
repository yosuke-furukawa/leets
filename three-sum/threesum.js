/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function(nums) {
    var res = [];
    var found = new Map();
    var dup = new Map();
    var seen = new Map();
    for (var i=0; i<nums.length;i++) {
        var x = nums[i];
        if (!dup.has(x)) {
            dup.set(x, true);
            for (var j=i+1;j<nums.length;j++) {
                var y = nums[j];
                var need = -x - y;
                if (seen.has(need) && seen.get(need) === i) {
                    var min = Math.min(x, y, need);
                    var max = Math.max(x, y, need);
                    if (!found.has(`${min},${max}`)) {
                        found.set(`${min},${max}`, true);
                        res.push([x, y, need]);
                    }
                }
                seen.set(y, i);
            }
        }
    }
    return res;
};
