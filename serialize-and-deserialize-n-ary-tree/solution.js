/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

class Codec {
  	constructor() {
        
    }
    
    /** 
     * @param {Node} root
     * @return {string}
     */
    // Encodes a tree to a single string.
    serialize(root) {
      if (root == null) {
        return null;
      }
      
      var results = [];
      var queue = [[root, results]];
      while (queue.length > 0) {
        const [node, result] = queue.shift();
        result.push(node.val);
        if (node.children.length > 0) {
          var array = [];
          result.push(array);
          for (const child of node.children) {
            queue.push([child, array]);
          }
        }
      }
      // console.log(JSON.stringify(results));
      return JSON.stringify(results);
    }
	
    /** 
     * @param {string} data 
     * @return {Node}
     */
    // Decodes your encoded data to tree.
    deserialize(data) {
      var arr = JSON.parse(data);
      if (arr == null || arr.length === 0) {
        return null;
      }
      var root = new Node(arr[0], []);
      if (arr.length === 1) {
        return root;
      }
      
      var queue = [[root, arr.slice(1)]];
      
      while (queue.length > 0) {
        var [node, arr] = queue.shift();
        var c = null
        if (node === root) {
          c = node;
        }
        for (var child of arr) {
          if (typeof child === "number") {
            c = new Node(child, []);
            node.children.push(c);
          }
          if (Array.isArray(child)) {
            queue.push([c, child]);
          }
        }
      }
      // console.log(root);
      
      return root;
    }
}

// Your Codec object will be instantiated and called as such:
// Codec codec = new Codec();
// codec.deserialize(codec.serialize(root));
