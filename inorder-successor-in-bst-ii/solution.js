/**
 * // Definition for a Node.
 * function Node(val) {
 *    this.val = val;
 *    this.left = null;
 *    this.right = null;
 *    this.parent = null;
 * };
 */

/**
 * @param {Node} node
 * @return {Node}
 */
var inorderSuccessor = function(node) {
  var found = false;
  var result = null;
  var helper = (n) => {
    if (n == null) {
      return null;
    }
    
    if (result) {
      return result;
    }

    var l;
    if (n.left) {
      l = helper(n.left);
    }
    // console.log(n.val);
    
    if (found && !result) {
      result = n;
      return result;
    }
        
    if (n.val === node.val) {
      found = true;
    }
        
    var r;
    if (n.right) {
      r = helper(n.right);
    }
    return l || r;
    
  };
  var head = node;
  var parent = head.parent;
  while(parent != null) {
    head = parent;
    parent = parent.parent;
  }
  // console.log(head.val);
  helper(head);
  return result;
};
