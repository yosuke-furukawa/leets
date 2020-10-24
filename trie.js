
class TrieNode {
  constructor() {
    this.children = new Map();
    this.word = null;
  }
}

class Trie {
  constructor() {
    this.root = new TrieNode();
  }
  insert(word) {
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
  }
  /**
   * Returns if the word is in the trie. 
   * @param {string} word
   * @return {boolean}
   */
  search(word) {
    var node = this.root
    for (const char of word) {
      if (node.children.has(char)) {
        node = node.children.get(char);
      } else {
        return false;
      }
    }
    return node.word === word;
  }

  /**
   * Returns if there is any word in the trie that starts with the given prefix. 
   * @param {string} prefix
   * @return {boolean}
   */
  startsWith(prefix) {
    var node = this.root
    for (const char of prefix) {
      if (node.children.has(char)) {
        node = node.children.get(char);
      } else {
        return false;
      }
    }
    return true;
  }
}

