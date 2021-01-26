/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val === undefined ? 0 : val;
 *    this.children = children === undefined ? [] : children;
 * };
 */

/**
 * @param {Node} root
 * @return {number}
 */
const diameter = function(root) {
  let deepDiameter = 0;
  const helper = (node) => {
    let depths = [];
    for (const n of node.children) {
      depths.push(helper(n));
    }
    depths.sort((a, b) => b-a);
    const first = depths[0] ?? 0;
    const second = depths[1] ?? 0;
    const diameter = first + second;
    deepDiameter = Math.max(diameter, deepDiameter);
    return first + 1;
  }
  helper(root, 0);
  return deepDiameter;
};
