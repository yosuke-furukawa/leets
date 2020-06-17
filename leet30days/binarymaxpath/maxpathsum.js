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
var maxPathSum = function(root) {
    var max = -Infinity;
    
    var traverse = function(node) {
        if (!node) return 0;
        var l = Math.max(traverse(node.left), 0);
        var r = Math.max(traverse(node.right), 0);
        
        max = Math.max(max, node.val + l + r);
        return node.val + Math.max(l, r);
    };
    traverse(root);
    return max;
};
