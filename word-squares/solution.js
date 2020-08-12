var wordSquares = function(words) {
  let result = []
  let trie = new Trie()
  for (let word of words) {
    trie.add(word)
  }
  
  findWordSquare(result, [], trie)
  return result
};


var findWordSquare = function (result, temp, trie) {
  if (temp.length > 0 && temp.length === temp[0].length) {
    result.push(temp)
    return
  }
  
  let prefix = ''
  let j = temp.length
  for (let i = 0; i < temp.length; i++) {
    prefix += temp[i][j]
  }
  
  let startWith = trie.startWith(prefix)
  for (let word of startWith) {
    findWordSquare(result, temp.concat([word]), trie)
  }
}

var Trie = function () {
  this.isWord = false
  this.children = new Map()
}

Trie.prototype.add = function (word) {
  let cur = this
  for (let i = 0; i < word.length; i++) {
    if (!cur.children.has(word[i])) {
      cur.children.set(word[i], new Trie())
    }
    cur = cur.children.get(word[i])
  }
  cur.isWord = true
}

Trie.prototype.startWith = function (prefix) {
  let cur = this
  for (let i = 0; i < prefix.length; i++) {
    if (cur.children.has(prefix[i])) {
      cur = cur.children.get(prefix[i])
    } else {
      return []
    }
  }
  
  let res = []
  var findWords = function (res, cur, str) {
    if (!cur.isWord) {
      for (let [key, val] of cur.children) {
        findWords(res, val, str + key)
      }
    } else {
      res.push(str)
    }
  }
  
  findWords(res, cur, prefix)
  return res
}
