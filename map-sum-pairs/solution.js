/**
 * Initialize your data structure here.
 */
var MapSum = function() {
  this.root = new TrieNode();
};

/** 
 * @param {string} key 
 * @param {number} val
 * @return {void}
 */
MapSum.prototype.insert = function(key, val) {
  var node = this.root;
  if (!this.search(key)) {
    for (var k of key) {
      // console.log(k, node.value);
      if (node.children.has(k)) {
        node = node.children.get(k);
        node.value += val;
      } else {
        var newNode = new TrieNode();
        node.children.set(k, newNode);
        node = newNode;
        node.value += val;
      }
    }
  } else {
    for (var k of key) {
      // console.log(k, node.value);
      if (node.children.has(k)) {
        node = node.children.get(k);
        node.value = val;
      } else {
        var newNode = new TrieNode();
        node.children.set(k, newNode);
        node = newNode;
        node.value = val;
      }
    }
  }
  node.word = key;
};

/** 
 * @param {string} prefix
 * @return {number}
 */
MapSum.prototype.sum = function(prefix) {
  var node = this.root;
  for (const char of prefix) {
    // console.log(char, node.value);
    if (node.children.has(char)) {
      node = node.children.get(char);
      // console.log(node);
    } else {
      return 0;
    }
  }
  return node.value;
};

MapSum.prototype.search = function(word) {
  var node = this.root;
  for (const char of word) {
    // console.log(char, node.value);
    if (node.children.has(char)) {
      node = node.children.get(char);
      // console.log(node);
    } else {
      return false;
    }
  }
  return node.word === word;
};

class TrieNode {
  constructor() {
    this.children = new Map();
    this.value = 0;
    this.word = null;
  }
}


/** 
 * Your MapSum object will be instantiated and called as such:
 * var obj = new MapSum()
 * obj.insert(key,val)
 * var param_2 = obj.sum(prefix)
 */
