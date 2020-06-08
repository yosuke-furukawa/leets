var sortedArrayToBST = function(nums) {
    if (nums.length === 0) {
        return null;
    }
    var mid = Math.floor(nums.length/2);
    var root = new TreeNode(nums[mid]);
    root.left = sortedArrayToBST(nums.slice(0, mid));
    root.right = sortedArrayToBST(nums.slice(mid+1, nums.length));
    return root;
};
