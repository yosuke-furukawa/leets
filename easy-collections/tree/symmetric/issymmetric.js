/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
var issym = function(left, right) {
    if (left === null && right === null) {
        return true;
    }
    if (left && !right) {
        return false
    }
    if (right && !left) {
        return false
    }
    if (left.val !== right.val) {
        return false;
    }
    return issym(left.left, right.right) && issym(right.left, left.right);
}

/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isSymmetric = function(root) {
    if (!root) {
      return true;
    }
    return issym(root.left, root.right);  
};
