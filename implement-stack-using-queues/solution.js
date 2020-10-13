/**
 * Initialize your data structure here.
 */
var MyStack = function() {
  this.queue1 = [];
  this.queue2 = [];
};

/**
 * Push element x onto stack. 
 * @param {number} x
 * @return {void}
 */
MyStack.prototype.push = function(x) {
  this.queue1.push(x);
};

/**
 * Removes the element on top of the stack and returns that element.
 * @return {number}
 */
MyStack.prototype.pop = function() {
  while (this.queue1.length > 1) {
    this.queue2.push(this.queue1.shift());
  }
  var x = this.queue1.shift();
  while (this.queue2.length > 0) {
    this.queue1.push(this.queue2.shift());
  }
  return x;
};

/**
 * Get the top element.
 * @return {number}
 */
MyStack.prototype.top = function() {
  return this.queue1[this.queue1.length-1];
};

/**
 * Returns whether the stack is empty.
 * @return {boolean}
 */
MyStack.prototype.empty = function() {
  return this.queue1.length === 0;
};

/** 
 * Your MyStack object will be instantiated and called as such:
 * var obj = new MyStack()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.empty()
 */
