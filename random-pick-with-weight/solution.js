/**
 * @param {number[]} w
 */
var Solution = function(w) {
  this.sum = w.reduce((a, c) => a+c);
  this.w = w.map((weight, index) => ({weight, index})).sort((a,b) => b.weight - a.weight);
  console.log(this.w);
};

/**
 * @return {number}
 */
Solution.prototype.pickIndex = function() {
  var num = Math.floor(Math.random() * this.sum) + 1;
  // console.log(num);
  var pre = 0;
  var cur = 0;
  for (var i=0;i<this.w.length;i++) {
    cur = pre + this.w[i].weight;
    if (pre < num && num <= cur) {
      return this.w[i].index;
    }
    pre = cur;
  }
};

/** 
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(w)
 * var param_1 = obj.pickIndex()
 */
