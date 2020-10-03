/**
 * @param {string[]} dictionary
 */
var ValidWordAbbr = function(dictionary) {
  this.map = new Map();
  for (var word of dictionary) {
    var h = word[0];
    var t = word[word.length - 1];
    var l = word.length - 2;
    var key = `${h}${l}${t}`;
    if (this.map.has(key)) {
      this.map.get(key).add(word);
      continue;
    }
    this.map.set(key, new Set([word]));
  }
};

/** 
 * @param {string} word
 * @return {boolean}
 */
ValidWordAbbr.prototype.isUnique = function(word) {
  var h = word[0];
  var t = word[word.length - 1];
  var l = word.length - 2;
  var key = `${h}${l}${t}`;
  var set = this.map.get(key);
  if (!set) {
    return true;
  }
  
  return set.has(word) && set.size === 1;
};

/** 
 * Your ValidWordAbbr object will be instantiated and called as such:
 * var obj = new ValidWordAbbr(dictionary)
 * var param_1 = obj.isUnique(word)
 */
