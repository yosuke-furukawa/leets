var helper = (node, level, results) => {
  if (node == null) {
    return;
  }
    
  if (results[level]) {
    results[level].push(node.val);
  } else {
    results[level] = [node.val];
  }
    
  node.children.forEach((c) => helper(c, level+1, results));
};

/**
 * @param {Node} root
 * @return {number[][]}
 */
var levelOrder = function(root) {
  var results = {};
  if (root == null) {
    return [];
  }
  
  helper(root, 0, results);
  results["length"] = Object.keys(results).length;
  return Array.from(results);
};
