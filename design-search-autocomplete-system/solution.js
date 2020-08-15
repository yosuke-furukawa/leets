/**
 * @param {string[]} sentences
 * @param {number[]} times
 */
var AutocompleteSystem = function(sentences, times) {
  this.trie = new Trie();
  this.sentencesMap = new Map();
  for (var i=0;i<sentences.length;i++) {
    this.trie.insert(sentences[i]);
    this.sentencesMap.set(sentences[i], times[i]);
  }
  this.chars = "";
};

/** 
 * @param {character} c
 * @return {string[]}
 */
AutocompleteSystem.prototype.input = function(c) {
  if (c === "#") {
    this.trie.insert(this.chars);
    var count = this.sentencesMap.get(this.chars) ?? 0;
    this.sentencesMap.set(this.chars, ++count);
    this.chars = "";
    return [];
  }
  this.chars += c;
  const candidates = this.trie.startsWith(this.chars).sort((a, b) => {
    var n = this.sentencesMap.get(b) - this.sentencesMap.get(a);
    if (n === 0) {
      return a.localeCompare(b);
    }
    return n;
  });
  return candidates.slice(0,3);
};

/** 
 * Your AutocompleteSystem object will be instantiated and called as such:
 * var obj = new AutocompleteSystem(sentences, times)
 * var param_1 = obj.input(c)
 */

class TrieNode {
  constructor() {
    this.children = new Map();
    this.word = null;
    this.wordMap = new Map();
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
    if (node.wordMap.has(char)) {
      node.wordMap.set(char, new Set([...node.wordMap.get(char), word]));
    } else {
      node.wordMap.set(char, new Set([word]));
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
  var candidates;
  for (const char of prefix) {
    if (node.children.has(char)) {
      node = node.children.get(char);
      candidates = node.wordMap.get(char).values();
    } else {
      return [];
    }
  }
  return [...candidates];
};

/** 
 * Your Trie object will be instantiated and called as such:
 * var obj = new Trie()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */
  
