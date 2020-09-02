/**
 * // Definition for a Node.
 * function Node(val, neighbors) {
 *    this.val = val === undefined ? 0 : val;
 *    this.neighbors = neighbors === undefined ? [] : neighbors;
 * };
 */

/**
 * @param {Node} node
 * @return {Node}
 */
var cloneGraph = function(node) {
  var map = new Map();
  var helper = (n) => {
    if (n == null) {
      return n;
    }
    if (map.has(n.val)) {
      return map.get(n.val);
    }
    
    const value = new Node(n.val);
    map.set(n.val, value);
    
    const neighbors = n.neighbors.map((neighbor) => helper(neighbor));
    value.neighbors = neighbors;
    return value;
  }
  
  return helper(node);
};
