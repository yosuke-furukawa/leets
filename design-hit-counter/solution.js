/**
 * Initialize your data structure here.
 */
var HitCounter = function() {
  this.array = [];
  this.current = 0;
};

/**
 * Record a hit.
        @param timestamp - The current timestamp (in seconds granularity). 
 * @param {number} timestamp
 * @return {void}
 */
HitCounter.prototype.hit = function(timestamp) {
  this.array.push(timestamp);
  this.current = timestamp;
  this.array = this.array.filter((t) => t > timestamp - 300);
};

/**
 * Return the number of hits in the past 5 minutes.
        @param timestamp - The current timestamp (in seconds granularity). 
 * @param {number} timestamp
 * @return {number}
 */
HitCounter.prototype.getHits = function(timestamp) {
  this.current = timestamp;
  this.array = this.array.filter((t) => t > timestamp - 300);
  return this.array.length;
};

/** 
 * Your HitCounter object will be instantiated and called as such:
 * var obj = new HitCounter()
 * obj.hit(timestamp)
 * var param_2 = obj.getHits(timestamp)
 */
