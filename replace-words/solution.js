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

/**
 * Returns if there is any word in the trie that starts with the given prefix. 
 * @param {string} prefix
 * @return {boolean}
 */
Trie.prototype.get = function(prefix) {
  var node = this.root;
  var hasPrefix = false;
  for (const char of prefix) {
    if (node.children.has(char) && node.word) {
      return node.word
    } else if (node.children.has(char)) {
      hasPrefix = true;
      node = node.children.get(char);
    } else if (node.word && hasPrefix) {
      return node.word;
    } else {
      return null;
    }
  }
  return node.word != null ? node.word : null;
};

/**
 * @param {string[]} dictionary
 * @param {string} sentence
 * @return {string}
 */
var replaceWords = function(dictionary, sentence) {
  const trie = new Trie();
  for (const word of dictionary) {
    trie.insert(word);
  }
  
  const results = [];
  const terms = sentence.split(" ");
  for (const term of terms) {
    var t = trie.get(term);
    if (t) {
      results.push(t);
    } else {
      results.push(term);
    }
  }
  
  return results.join(" ");
};

