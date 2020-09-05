/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root1
 * @param {TreeNode} root2
 * @return {number[]}
 */
var getAllElements = function(root1, root2) {
  var list1 = [];
  var list2 = [];
  var helper = (node, list) => {
    if (node == null) {
      return;
    }
    
    helper(node.left, list);
    list.push(node.val);
    helper(node.right, list);
  };
  
  helper(root1, list1);
  helper(root2, list2);
  var list = [...list1, ...list2].sort((a, b) => a-b);
  return list;
};
