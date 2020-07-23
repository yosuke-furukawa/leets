class TrieNode {
  constructor() {
    this.children = new Map();
    this.word = null;
  }
}

/**
 * Initialize your data structure here.
 */
var Trie = function() {
  this.root = new TrieNode();
};

/**
 * Inserts a word into the trie. 
 * @param {string} word
 * @return {void}
 */
Trie.prototype.insert = function(word) {
  var node = this.root;
  for (const char of word) {
    if (node.children.has(char)) {
      node = node.children.get(char);
    } else {
      var newNode = new TrieNode();
      node.children.set(char, newNode);
      node = newNode; 
    }
  }
  node.word = word;
};

/**
 * Returns if the word is in the trie. 
 * @param {string} word
 * @return {boolean}
 */
Trie.prototype.search = function(word) {
  var node = this.root
  for (const char of word) {
    if (node.children.has(char)) {
      node = node.children.get(char);
    } else {
      return false;
    }
  }
  return node.word === word;
};

/**
 * Returns if there is any word in the trie that starts with the given prefix. 
 * @param {string} prefix
 * @return {boolean}
 */
Trie.prototype.startsWith = function(prefix) {
  var node = this.root
  for (const char of prefix) {
    if (node.children.has(char)) {
      node = node.children.get(char);
    } else {
      return false;
    }
  }
  return true;
};

/** 
 * Your Trie object will be instantiated and called as such:
 * var obj = new Trie()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */
