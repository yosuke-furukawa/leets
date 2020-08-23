
/**
 * @param {string[]} words
 */
var StreamChecker = function(words) {
  this.trie = new Trie();
  for (const word of words) {
    this.trie.insert(word);
  }
  this.currentNodes = [];
};

/** 
 * @param {character} letter
 * @return {boolean}
 */
StreamChecker.prototype.query = function(letter) {
  if (this.trie.startsWith(letter)) {
    this.currentNodes.push(this.trie.root);
  }
  
  let found = false;
  for (var i=0; i<this.currentNodes.length;i++) {
    const currentNode = this.currentNodes[i];
    const node = this.trie.startsWithNode(currentNode, letter);
    if (node == null) {
      this.currentNodes.splice(i, 1);
      i--;
      continue;
    }
    if (node.word == null) {
      this.currentNodes[i] = node;
      continue;
    }
  
    if (node.word) {
      this.currentNodes[i] = node;
      found = true;
      continue;
    }
  }
  return found;
};

/** 
 * Your StreamChecker object will be instantiated and called as such:
 * var obj = new StreamChecker(words)
 * var param_1 = obj.query(letter)
 */

class TrieNode {
  constructor() {
    this.children = new Map();
    this.word = null;
  }
}

var Trie = function() {
  this.root = new TrieNode();
};

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

Trie.prototype.startsWithNode = function(root, letter) {
  var node = root;
  
  if (node.children.has(letter)) {
    node = node.children.get(letter);
    return node;
  }
  return null;
};

