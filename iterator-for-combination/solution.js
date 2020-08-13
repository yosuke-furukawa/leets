function nextCombination(sub) {
    var x = sub & -sub
    var y = sub + x;
    return (((sub & ~y) / x) >> 1) | y;
}

/**
 * @param {string} characters
 * @param {number} combinationLength
 */
var CombinationIterator = function(characters, combinationLength) {
  var bit = (1<<combinationLength)-1;
  this.combinations = [];
  for (;bit < (1<<characters.length); bit = nextCombination(bit)) {
    var s = "";
    for (var i = 0; i < characters.length; i++) {
      if (bit & (1<<i)) {
        s += characters[i];
      }
    }
    this.combinations.push(s);
  }
  this.combinations = this.combinations.sort();
};

/**
 * @return {string}
 */
CombinationIterator.prototype.next = function() {
  return this.combinations.shift();
};

/**
 * @return {boolean}
 */
CombinationIterator.prototype.hasNext = function() {
  return this.combinations.length > 0;
};

/** 
 * Your CombinationIterator object will be instantiated and called as such:
 * var obj = new CombinationIterator(characters, combinationLength)
 * var param_1 = obj.next()
 * var param_2 = obj.hasNext()
 */
