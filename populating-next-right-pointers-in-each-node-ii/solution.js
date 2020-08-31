/**
 * // Definition for a Node.
 * function Node(val, left, right, next) {
 *    this.val = val === undefined ? null : val;
 *    this.left = left === undefined ? null : left;
 *    this.right = right === undefined ? null : right;
 *    this.next = next === undefined ? null : next;
 * };
 */

/**
 * @param {Node} root
 * @return {Node}
 */
var connect = function(root) {
  var map = new Map();
  var helper = (node, level, arr) => {
    if (node == null) {
      return;
    }
    if (map.has(level)) {
      arr = map.get(level);
    }
    arr.push(node);
    map.set(level, arr);
    var newarr = [];
    helper(node.left, level + 1, newarr);
    
    helper(node.right, level + 1, newarr);
  };
  helper(root, 0, []);
  for (const key of map.keys()) {
    const arr = map.get(key);
    for (var i=0;i<arr.length - 1;i++) {
      var node = arr[i];
      var next = arr[i+1];
      node.next = next;
    }
  }
  
  return root;
};
