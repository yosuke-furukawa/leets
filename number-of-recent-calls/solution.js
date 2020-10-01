
var RecentCounter = function() {
  this.scale = 3000;
  this.queue = [];
};

/** 
 * @param {number} t
 * @return {number}
 */
RecentCounter.prototype.ping = function(t) {
  this.queue.push(t);
  const rangeMax = t;
  const rangeMin = t - this.scale;
  this.queue = this.queue.filter((q) => q >= rangeMin && q <= rangeMax);
  return this.queue.length;
};

/** 
 * Your RecentCounter object will be instantiated and called as such:
 * var obj = new RecentCounter()
 * var param_1 = obj.ping(t)
 */
