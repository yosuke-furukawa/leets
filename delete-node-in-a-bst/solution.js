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
 * @param {number} key
 * @return {TreeNode}
 */
var deleteNode = function(root, key) {
  if (root == null) {
    return root;
  }
  var deleted = null;
  var parentOfRoot = new TreeNode(-1, root);
  var deletedParent = null;
  var leftOrRight = null;
  var helper = (node, parent) => {
    if (node == null) {
      return;
    }
    if (deleted) {
      return;
    }
    
    if (node.val === key) {
      node.val = -1;
      deletedParent = parent;
      if (deletedParent.left == node) {
        leftOrRight = "left";
      } else {
        leftOrRight = "right";
      }
      deleted = node;
      return;
    }
    
    helper(node.left, node);
    helper(node.right, node);
  }
  helper(root, parentOfRoot);

  if (deleted == null) {
    return root;
  }
  var left = deleted.left;
  var right = deleted.right;
  var parent;
  var queue = [];
  if (right) {
    parent = right;
    if (left) {
      queue = [left];
    }
  } else if (left) {
    parent = left;
    if (right) {
      queue = [right];
    }
  } else {
    parent = null;
  }
  while (queue.length > 0) {
    var node = queue.pop();
    
    if (node.left) {
      queue.unshift(node.left);
    }
    
    if (node.right) {
      queue.unshift(node.right);
    }
    // traverse
    var p = parent;
    while (p != null) {
      if (p.val < node.val) {
        if (p.right == null) {
          p.right = new TreeNode(node.val);
          break;
        }
        p = p.right;
      } else {
        if (p.left == null) {
          p.left = new TreeNode(node.val);
          break;
        }
        p = p.left;
      }
    }
  }
  deletedParent[leftOrRight] = parent;
  return parentOfRoot.left;
};
