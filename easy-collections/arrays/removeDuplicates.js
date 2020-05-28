var removeDuplicates = function(nums) {
    for (var i=0; i<nums.length; i++) {
        if (nums[i] === nums[i+1]) {
            nums.splice(i, 1);
            i--;
        }
    }
    return nums.length;
};
