/**
 * @param {number} n
 */
var OrderedStream = function(n) {
  this.array = new Array(n+1);
  this.ptr = 1;
  this.set = new Set();
};

/** 
 * @param {number} id 
 * @param {string} value
 * @return {string[]}
 */
OrderedStream.prototype.insert = function(id, value) {
  this.array[id] = value;
  // console.log(this.ptr, this.array);
  var results = [];
  for (var i=1;i<=this.ptr;i++) {
    var temp = this.array[i];
    if (temp == null) {
      break;
    }
    if (temp != null && !this.set.has(temp)) {
      results.push(temp);
      this.set.add(temp);
    }
  }
  this.ptr++;
  return results;
};

/** 
 * Your OrderedStream object will be instantiated and called as such:
 * var obj = new OrderedStream(n)
 * var param_1 = obj.insert(id,value)
 */
