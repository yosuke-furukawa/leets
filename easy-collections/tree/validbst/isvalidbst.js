var dfs = function(root, lower, upper) {
  if (!root) return true;
  if (lower != null && lower >= root.val) {
    return false;
  }
  if (upper != null && upper <= root.val) {
    return false
  }
  return dfs(root.left, lower, root.val) && dfs(root.right, root.val, upper);
}

var isValidBST = function(root) {
  return dfs(root, null, null);
};
