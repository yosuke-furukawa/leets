/**
 * @param {string} compressedString
 */
var StringIterator = function(compressedString) {
  var result = [];
  for (var i=0;i<compressedString.length;) {
    var c = compressedString[i];
    var num = 0;
    for (var j=i+1;j<compressedString.length;) {
      if (+compressedString[j] != compressedString[j]) {
        break;
      }
      num = num * 10 + Number(compressedString[j]);
      j++;
    }
    result.push([c, num]);
    i = j;
  }
  this.result = result;
};

/**
 * @return {character}
 */
StringIterator.prototype.next = function() {
  const [c, num] = this.result.shift() ?? [" ", 1];
  if (num > 1) {
    this.result.unshift([c, num-1]);
  }
  return c;
};

/**
 * @return {boolean}
 */
StringIterator.prototype.hasNext = function() {
  return this.result.length > 0
};

/** 
 * Your StringIterator object will be instantiated and called as such:
 * var obj = new StringIterator(compressedString)
 * var param_1 = obj.next()
 * var param_2 = obj.hasNext()
 */
