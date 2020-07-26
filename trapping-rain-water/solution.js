/**
 * @param {number[]} height
 * @return {number}
 */

var trap = function(height) {
  var rain = 0;
  var left = 0;
  var right = height.length;
  var raincandidate = 0;
  var lefth = 0;
  var righth = 0;
  for (var i=0;i<height.length;) {
    var h = height[i];
    if (lefth === 0 && h < height[i-1]) {
      left = i-1;
      lefth = height[left];
    }
    if (lefth > 0) {
      // find heigher than left and max
      var max = 0, maxindex = 0;
      for (var j=left+1;j<right;j++) {
        var hn = height[j];
        if (max < hn) {
          max = hn;
          righth = max;
          maxindex = j;
        }
        if (hn >= lefth) {
          break;
        }
      }
      right = maxindex;
    }
    if (righth > 0) {
      var watermark = Math.min(lefth, righth);
      for (i=left+1;i<right;i++) {
        rain += watermark - height[i];
      }
      lefth = righth;
      left = right;
      right = height.length;
      righth = 0;
      continue;
    }
    i++
  }
  return rain;
};
