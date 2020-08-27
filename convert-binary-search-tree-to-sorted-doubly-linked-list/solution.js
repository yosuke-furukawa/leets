/**
 * // Definition for a Node.
 * function Node(val, left, right) {
 *      this.val = val;
 *      this.left = left;
 *      this.right = right;
 *  };
 */

/**
 * @param {Node} root
 * @return {Node}
 */
var treeToDoublyList = function(root) {
  if (root == null) {
    return root;
  }
  var list = [];
  var dfs = (node) => {
    if (node == null) {
      return;
    }
    dfs(node.left);
    list.push(node);
    dfs(node.right);
  };
  dfs(root);
  var head = list[0];
  head.left = list[list.length-1];
  var node = head;
  for (var i=1;i<list.length;i++) {
    var next = list[i];
    node.right = next;
    next.left = node;
    node = next;
  }
  list[list.length-1].right = head;
  return head;
};
