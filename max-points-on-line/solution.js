/**
 * @param {number[][]} points
 * @return {number}
 */
var N = 1000000000000;


var maxPoints = function(points) {
  var map = new Map();
  
  if (points.length === 0) {
    return 0;
  }
  var max = 0;
  var calcA = 0;
  var calcB = 0;
  var calcC = 1;
  for (var i = 0; i<points.length;i++) {
    var [x1, y1] = points[i];
    for (var j=i+1; j<points.length;j++) {
      const [x2, y2] = points[j];
      var a = (y1-y2) / (x1-x2);
      var b = ((x1-x2)*y1 - (y1-y2)*x1) / (x1-x2);
      if (a === 0.9999999894638303) {
        break;
      }
      var key;
      if (Number.isFinite(a) && Number.isFinite(b)) {
        key = `y=${a}x+${b}`;
      } else if (x1 !== x2) {
        key = `y=${y1}`;
      } else if (y1 !== y2) {
        key = `x=${x1}`;
      } else {
        key = `y=0,x=0`;
      }
      if (map.has(key)) {
        var count = map.get(key);
        count++;
        if (max < count) {
          max = count;
          if (Number.isFinite(a) && Number.isFinite(b)) {
            calcA = a;
            calcB = b;
            calcC = 1;
          } else if (x1 !== x2) {
            calcA = 0;
            calcB = y1;
            calcC = 1;
          } else if (y1 !== y2) {
            calcA = 1;
            calcB = -x1;
            calcC = 0;
          } else {
            calcA = 0;
            calcB = 0;
            calcC = 0;
          }
        }
        map.set(key, count);
        continue;
      }
      if (max === 0) {
        if (Number.isFinite(a) && Number.isFinite(b)) {
          calcA = a;
          calcB = b;
          calcC = 1;
        } else if (x1 !== x2) {
          calcA = 0;
          calcB = y1;
          calcC = 1;
        } else if (y1 !== y2) {
          calcA = 1;
          calcB = -x1;
          calcC = 0;
        } else {
          calcA = 0;
          calcB = 0;
          calcC = 0;
        }
        max = 1;
      }
      map.set(key, 2);
    }
  }
  return points.filter(([x, y]) => (Math.round((y*calcC - calcA*x - calcB)*N)/N) === 0).length;
};
