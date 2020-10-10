/**
 * @param {number[][]} points
 * @return {number}
 */
var findMinArrowShots = function(points) {
  if (points.length <= 1) {
    return points.length;
  }
  var sorted = points.sort((a, b) => { 
    var s = a[0] - b[0];
    return s===0 ? a[1]-b[1] : s;
  });
  
  for (var i=0;i<sorted.length-1;i++) {
    var balloon1 = sorted[i];
    var balloon2 = sorted[i+1];
    if (balloon1[0] <= balloon2[1] && balloon1[1] >= balloon2[0]) {
      sorted.splice(i,2, [Math.max(balloon2[0],balloon1[0]), Math.min(balloon1[1], balloon2[1])]);
      i--;
    }
  }
  return sorted.length;
};
