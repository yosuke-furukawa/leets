/**
 * Initialize your data structure here.
 */
var TwoSum = function() {
  this.map = new Map();
  this.i = 0;
};

/**
 * Add the number to an internal data structure.. 
 * @param {number} number
 * @return {void}
 */
TwoSum.prototype.add = function(number) {
  if (this.map.has(number)) {
    this.map.get(number).push(this.i);
    this.i++;
    return;
  }
  this.map.set(number, [this.i]);
  this.i++;
};

/**
 * Find if there exists any pair of numbers which sum is equal to the value. 
 * @param {number} value
 * @return {boolean}
 */
TwoSum.prototype.find = function(value) {
  for (var key of this.map.keys()) {
    const entries = this.map.get(key);
    const target = value - key;  
    const exists = this.map.get(target);
    if (Boolean(exists) && exists !== entries) {
      return true;
    } else if (exists === entries && exists.length > 1) {
      return true;
    }
  }
  return false;
};

/** 
 * Your TwoSum object will be instantiated and called as such:
 * var obj = new TwoSum()
 * obj.add(number)
 * var param_2 = obj.find(value)
 */
