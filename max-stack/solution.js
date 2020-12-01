/**
 * initialize your data structure here.
 */
var MaxStack = function() {
  this.stack = [];
};

/** 
 * @param {number} x
 * @return {void}
 */
MaxStack.prototype.push = function(x) {
  this.stack.push(x);
};

/**
 * @return {number}
 */
MaxStack.prototype.pop = function() {
  return this.stack.pop();
};

/**
 * @return {number}
 */
MaxStack.prototype.top = function() {
  return this.stack[this.stack.length - 1];
};

/**
 * @return {number}
 */
MaxStack.prototype.peekMax = function() {
  var s = [...this.stack].sort((a, b) => a - b);
  return s[this.stack.length-1];
};

/**
 * @return {number}
 */
MaxStack.prototype.popMax = function() {
  var s = [...this.stack].sort((a, b) => a - b);
  var max = s[this.stack.length-1];
  this.stack.splice(this.stack.lastIndexOf(max), 1);
  return max;
};

/** 
 * Your MaxStack object will be instantiated and called as such:
 * var obj = new MaxStack()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.peekMax()
 * var param_5 = obj.popMax()
 */
