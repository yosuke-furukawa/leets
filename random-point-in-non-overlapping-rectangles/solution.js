/**
 * @param {number[][]} rects
 */
var Solution = function(rects) {
  this.rects = rects;
  this.areas = [];
  this.sum = 0;
  for (var rect of rects) {
    var width = Math.abs(rect[2] - rect[0]) + 1;
    var height = Math.abs(rect[3] - rect[1]) + 1;
    var temp = width * height;
    this.sum += temp;
    this.areas.push(this.sum);
  }
};

/**
 * @return {number[]}
 */
Solution.prototype.pick = function() {
  const randArea = Math.random() * this.sum;
  let i = 0;
  for (i=0;i<this.areas.length;++i) {
    if (randArea <= this.areas[i]) {
      break;
    }
  }
  const rect = this.rects[i];
  
  const xmin = rect[0];
  const xmax = rect[2];
  const ymin = rect[1];
  const ymax = rect[3];

  const x = Math.floor(Math.random() * (xmax - xmin + 1)) + xmin;
  const y = Math.floor(Math.random() * (ymax - ymin + 1)) + ymin;
  return [x, y];
};

/** 
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(rects)
 * var param_1 = obj.pick()
 */
