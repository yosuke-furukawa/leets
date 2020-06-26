/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

/**
 * Encodes a tree to a single string.
 *
 * @param {TreeNode} root
 * @return {string}
 */
var serialize = function(root) {
  var result = [];
  var queue = [root];
  var head = root;
  while(queue.length > 0) {
    var entry = queue.pop();
    if (entry == null) {
      result.push(null);
      continue;
    }
    result.push(entry.val);
    queue.unshift(entry.left);
    queue.unshift(entry.right);
  }
  for (var i=result.length-1;i>=0;i--) {
    if (result[i] == null) {
      result.pop();
    } else {
      break;
    }
  }
  console.log(result);
  return `[${result.join(",")}]`;
};

/**
 * Decodes your encoded data to tree.
 *
 * @param {string} data
 * @return {TreeNode}
 */
var deserialize = function(data) {
  var array = data.substring(1, data.length-1).split(",");
  if (array.filter(Boolean).length === 0) {
    return null;
  }
  var depth = 0;
  var head = new TreeNode(array.shift());
  var parents = [head];
  while (array.length > 0) {
    var size = 2 ** depth;
    for (var i=0; i<Math.ceil(size/2); i++) {
      var l = array.shift();
      var r = array.shift();
      
      var left = l ? new TreeNode(l) : null;
      var right = r ? new TreeNode(r) : null;
      var parent = parents.shift();
      while (!parent && parents.length > 0) {
        parent = parents.shift();
      }
      if (!parent) {
        break;
      }
      parent.left = left;
      parent.right = right;
      parents.push(left, right);
    }
    depth++;
  }
  return head;
};

/**
 * Your functions will be called as such:
 * deserialize(serialize(root));
 */
