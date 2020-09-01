/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function(root, p, q) {
  var pstack;
  var qstack;
  var helper = (node, stack) => {
    // console.log({pstack ,qstack});
    
    if (node == null) {
      return;
    }
    
    // console.log(node.val, p, q);
    if (node === p) {
      let newStack = [...stack];
      newStack.push(node);
      pstack = newStack;
    } else if (node === q) {
      let newStack = [...stack];
      newStack.push(node);
      qstack = newStack;
    }
    
    if (pstack && qstack) {
      return;
    }
    
    var newStack = [...stack];
    newStack.push(node);
    helper(node.left, newStack);
    helper(node.right, newStack);
  };
  helper(root, []);
  // console.log(pstack, qstack);
  if (pstack == null || qstack == null) {
    return;
  }
  for (var i=0;i<Math.max(pstack.length, qstack.length);i++) {
    if (pstack[i] === qstack[i]) {
      continue;
    } else {
      return pstack[i-1];
    }
  }
};
