var FrontMiddleBackQueue = function() {
  this.queue = [];
};

/** 
 * @param {number} val
 * @return {void}
 */
FrontMiddleBackQueue.prototype.pushFront = function(val) {
  // console.log(this.queue);
  this.queue.unshift(val);
};

/** 
 * @param {number} val
 * @return {void}
 */
FrontMiddleBackQueue.prototype.pushMiddle = function(val) {
  // console.log(this.queue);
  this.queue.splice(Math.floor(this.queue.length/2),0,val);
};

/** 
 * @param {number} val
 * @return {void}
 */
FrontMiddleBackQueue.prototype.pushBack = function(val) {
  // console.log(this.queue);
  this.queue.push(val);
};

/**
 * @return {number}
 */
FrontMiddleBackQueue.prototype.popFront = function() {
  // console.log(this.queue);
  return this.queue.shift() ?? -1;
};

/**
 * @return {number}
 */
FrontMiddleBackQueue.prototype.popMiddle = function() {
  // console.log(this.queue);
  // console.log(Math.floor(this.queue.length-1/2));
  var a = this.queue.splice(Math.floor((this.queue.length-1)/2), 1)?.[0];
  // console.log(a);
  return a ?? -1;
};

/**
 * @return {number}
 */
FrontMiddleBackQueue.prototype.popBack = function() {
  // console.log(this.queue);
  return this.queue.pop() ?? -1;
};

/** 
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * var obj = new FrontMiddleBackQueue()
 * obj.pushFront(val)
 * obj.pushMiddle(val)
 * obj.pushBack(val)
 * var param_4 = obj.popFront()
 * var param_5 = obj.popMiddle()
 * var param_6 = obj.popBack()
 */
