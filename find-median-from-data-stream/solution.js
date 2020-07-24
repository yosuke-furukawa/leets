/**
 * initialize your data structure here.
 */
var MedianFinder = function() {
  this.list = [];
  this.median = 0;
};

/** 
 * @param {number} num
 * @return {void}
 */
MedianFinder.prototype.addNum = function(num) {
  this.list.push(num);
  this.list = this.list.sort((a, b) => a-b);
  if ((this.list.length%2) === 1) {
    this.median = this.list[Math.floor(this.list.length/2)];
  } else {
    var p = Math.floor(this.list.length/2);
    this.median = (this.list[p-1] + this.list[p])/2;
  }
};

/**
 * @return {number}
 */
MedianFinder.prototype.findMedian = function() {
  return this.median;
};

/** 
 * Your MedianFinder object will be instantiated and called as such:
 * var obj = new MedianFinder()
 * obj.addNum(num)
 * var param_2 = obj.findMedian()
 */
