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
 * @param {number[]} arr
 * @return {boolean}
 */
var isValidSequence = function(root, arr) {
    var target = arr[0];
    arr.shift();
    if (root === null) {
        return false;
    }
    if (target !== root.val) {
        return false;
    }
    if (target === root.val && root.left === null && root.right === null && arr.length === 0) {
        return true;
    }
    return isValidSequence(root.left, [...arr]) || isValidSequence(root.right, [...arr]);
};
