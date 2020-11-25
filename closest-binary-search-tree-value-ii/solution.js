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
 * @param {number} target
 * @param {number} k
 * @return {number[]}
 */
var closestKValues = function(root, target, k) {
  var sorted = [];
  var helper = (node) => {
    if (node == null) {
      return;
    }
    
    helper(node.left);
    sorted.push(node.val);
    helper(node.right);
  };
  helper(root);
  sorted = sorted.sort((a, b) => a-b);
  if (sorted.length === 1) {
    return [sorted[0]];
  }
  sorted.unshift(-Infinity);
  sorted.push(Infinity);
  
  var result = [];
  for (var j=0;j<k;j++) {
    var diff = Math.abs(target - sorted[0]);
    for (var i=1;i<sorted.length;i++) {
      var n = sorted[i];
      if (Math.abs(target - n) < diff) {
        diff = Math.abs(target - n);
      } else {
        var p = sorted.splice(i-1, 1);
        result.push(p);
        break;
      }
    }
  }
  
  return result;
};
