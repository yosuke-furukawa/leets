/**
 * @param {number[]} p1
 * @param {number[]} p2
 * @param {number[]} p3
 * @param {number[]} p4
 * @return {boolean}
 */
var validSquare = function(p1, p2, p3, p4) {
  var points = [p1,p2,p3,p4];
  [p1, p2, p3, p4] = points.sort((a, b) => {
    var x = a[0] - b[0];
    var y = a[1] - b[1];
    if (x === 0) {
      return y;
    }
    return x;
  });
  
  var degree1 = calcDegree(p1, p2, p3);    
  var degree2 = calcDegree(p2, p4, p1);
  var degree3 = calcDegree(p3, p1, p4);
  var degree4 = calcDegree(p4, p3, p2);
  
  var len1 = calcLength(p1,p2);
  var len2 = calcLength(p1,p3);
  var len3 = calcLength(p2,p4);
  var len4 = calcLength(p3,p4);

  return degree1 === 90 && degree2 === 90 && degree3 === 90 && degree4 === 90 && len1 === len2 && len2 === len3 && len3 === len4;
};

function calcLength(p1, p2) {
  return Math.sqrt(Math.pow(p1[0]-p2[0],2) + Math.pow(p1[1]-p2[1], 2));
}

function calcDegree(p1, p2, p3) {
  return Math.round(Math.abs((Math.atan2(p2[1]-p1[1], p2[0]-p1[0]) * (180 / Math.PI) - Math.atan2(p3[1]-p1[1], p3[0]-p1[0]) * (180 / Math.PI))))%180;
}
