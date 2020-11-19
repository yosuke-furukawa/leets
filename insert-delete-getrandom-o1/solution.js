/**
 * Initialize your data structure here.
 */
var RandomizedCollection = function() {
  this.array = [];
};

/**
 * Inserts a value to the collection. Returns true if the collection did not already contain the specified element. 
 * @param {number} val
 * @return {boolean}
 */
RandomizedCollection.prototype.insert = function(val) {
  var has = this.array.indexOf(val) >= 0;
  this.array.push(val);
  return !has;
};

/**
 * Removes a value from the collection. Returns true if the collection contained the specified element. 
 * @param {number} val
 * @return {boolean}
 */
RandomizedCollection.prototype.remove = function(val) {
  var len = this.array.length;
  var index = this.array.indexOf(val);
  if (index < 0) {
    return false;
  }
  this.array.splice(index, 1);
  return true; 
};

/**
 * Get a random element from the collection.
 * @return {number}
 */
RandomizedCollection.prototype.getRandom = function() {
  var num = Math.floor(Math.random() * this.array.length);
  return this.array[num];
};

/** 
 * Your RandomizedCollection object will be instantiated and called as such:
 * var obj = new RandomizedCollection()
 * var param_1 = obj.insert(val)
 * var param_2 = obj.remove(val)
 * var param_3 = obj.getRandom()
 */
