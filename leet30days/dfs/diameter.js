/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var diameterOfBinaryTree = function(root) {
    let res = 0;
    const countDepth = function(root) {
        if (!root) return 0; 
        let left = countDepth(root.left);
        let right = countDepth(root.right);
        res = Math.max(res, left + right);
        return Math.max(left,right) + 1;
    }
    countDepth(root);
    return res;
};
