/**
 * @param {number[]} rec1
 * @param {number[]} rec2
 * @return {boolean}
 */
var isRectangleOverlap = function(rec1, rec2) {
  if ((rec1[0] < rec2[2] && rec1[2] > rec2[0] && rec1[1] < rec2[3] && rec1[3] > rec2[1]) ||
    (rec1[0] > rec2[2] && rec1[2] < rec2[0] && rec1[1] > rec2[3] && rec1[3] < rec2[1])) {
    var x1 = Math.max(rec2[0], rec1[0]);
    var x2 = Math.min(rec2[2], rec1[2]);
    
    var y1 = Math.max(rec2[1], rec1[1]);
    var y2 = Math.min(rec2[3], rec1[3]);
    var xlen = Math.abs(x2 - x1);
    var ylen = Math.abs(y2 - y1);
    if ((xlen * ylen) > 0) {
      return true;
    }
  }
  return false;
};
