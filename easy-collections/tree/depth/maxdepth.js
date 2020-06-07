class TreeNode {
  constructor(val, left, right) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
  static add(trees, root, i=0) {
    if (i<trees.length) {
      if (trees[i] === null) {
        return null;
      }
      var temp = new TreeNode(trees[i]);
      root = temp;
      root.left = TreeNode.add(trees, root.left, 2 * i + 1);
      root.right = TreeNode.add(trees, root.right, 2 * i + 2);
      console.log({root});
    }
    return root;
  }
}

var maxDepth = function(root) {
  if (root == null) {
    return 0;
  }
  var left = maxDepth(root.left);
  var right = maxDepth(root.right);
  console.log({left, right});

  return Math.max(left, right) + 1;
};

var tree = new TreeNode();
var trees = TreeNode.add([3,9,20,null,null,15,7,1,2], tree, 0);
console.log(maxDepth(trees));;

