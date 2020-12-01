/**
 * Initialize your data structure here.
 */
var AllOne = function() {
  this.map = new Map();
  this.keys = [];
};

/**
 * Inserts a new key <Key> with value 1. Or increments an existing key by 1. 
 * @param {string} key
 * @return {void}
 */
AllOne.prototype.inc = function(key) {
  if (this.map.has(key)) {
    this.map.set(key, this.map.get(key) + 1);
    return;
  }
  this.map.set(key, 1);
};

/**
 * Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. 
 * @param {string} key
 * @return {void}
 */
AllOne.prototype.dec = function(key) {
   if (this.map.has(key)) {
     this.map.set(key, this.map.get(key) - 1);
     if (this.map.get(key) === 0) {
       this.map.delete(key);
     }
   }
};

/**
 * Returns one of the keys with maximal value.
 * @return {string}
 */
AllOne.prototype.getMaxKey = function() {
  this.keys = Array.from(this.map.entries()).sort((a, b) => b[1] - a[1]);
  return this.keys[0]?.[0] ?? "";
};

/**
 * Returns one of the keys with Minimal value.
 * @return {string}
 */
AllOne.prototype.getMinKey = function() {
  this.keys = Array.from(this.map.entries()).sort((a, b) => b[1] - a[1]);
  return this.keys[this.keys.length-1]?.[0] ??  "";
};

/** 
 * Your AllOne object will be instantiated and called as such:
 * var obj = new AllOne()
 * obj.inc(key)
 * obj.dec(key)
 * var param_3 = obj.getMaxKey()
 * var param_4 = obj.getMinKey()
 */
