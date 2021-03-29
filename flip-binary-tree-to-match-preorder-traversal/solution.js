const flipMatchVoyage = function (root, V) {
    let ans = [], vix = 0
    const dfs = node => {
        if (!node || ans[0] === -1) return
        if (node.val !== V[vix++]) ans = [-1]
        else if (node.left && node.left.val !== V[vix]) {
            ans.push(node.val)
            dfs(node.right)
            dfs(node.left)
        } else {
            dfs(node.left)
            dfs(node.right)
        }
    }
    dfs(root)
    return ans
};
