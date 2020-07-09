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
var widthOfBinaryTree = function(root) {
  
    let max = 1
    let stack = []
    
    const width = (root, level, pos) => {
        
        if (root == null) {
            return
        }

        if (level >= stack.length) {
            stack.push(pos)
        }
        else 
            max = Math.max(max, pos - stack[level] + 1)

        width (root.left, level + 1, 2 * pos)
        width (root.right, level + 1, 2 * pos +1)
    }
    
    width(root, 0, 1)
    
    return max
};
