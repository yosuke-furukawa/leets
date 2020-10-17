/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

class Codec {
  	constructor() {
    }
    
    /** 
     * @param {Node} root
     * @return {TreeNode}
     */
    // Encodes an n-ary tree to a binary tree.
    encode(root) {
      if (root == null) {
        return null;
      }
      
      var newRoot = new TreeNode(root.val);
      var head = [
        newRoot,
        root
      ];
      
      var queue = [];
      queue.push(head);
      
      while(queue.length > 0) {
        var pair = queue.shift();
        var bNode = pair[0];
        var nNode = pair[1];
        
        var prevBNode = null;
        var headBNode = null;
        for (const nChild of nNode.children) {
          var newBNode = new TreeNode(nChild.val);
          if (prevBNode == null) {
            headBNode = newBNode;         
          } else {
            prevBNode.right = newBNode;
          }
          prevBNode = newBNode;
          
          var nextEntry = [newBNode, nChild];
          queue.push(nextEntry);
        }
        bNode.left = headBNode;
      }
      
      return newRoot;
    }
	
    /** 
     * @param {TreeNode} root 
     * @return {Node}
     */
    // Decodes your binary tree to an n-ary tree.
    decode(root) {
		  if (root == null) {
        return null;
      }
      
      var newRoot = new Node(root.val, []);
      
      var queue = [];
      var head = [newRoot, root];
      queue.push(head);
      
      while (queue.length > 0) {
        var entry = queue.shift();
        var nNode = entry[0];
        var bNode = entry[1];
        
        var firstChild = bNode.left;
        var sibling = firstChild;
        
        while (sibling != null) {
          var nChild = new Node(sibling.val, []);
          
          nNode.children.push(nChild);
          
          var nextEntry = [nChild, sibling];
          queue.push(nextEntry);
          
          sibling = sibling.right;
        }
      }
      
      return newRoot;
    }
}

/*
* Your Codec object will be instantiated and called as such:
* codec = Codec()
* codec.decode(codec.encode(root))
*/
