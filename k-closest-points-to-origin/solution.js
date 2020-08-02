/**
 * @param {number[][]} points
 * @param {number} K
 * @return {number[][]}
 */
var kClosest = function(points, K) {
  var newpoints = points.map((p) => p.push(p[0]**2+p[1]**2) && p);
  
  return newpoints.sort((p1, p2) => (p1[2] - p2[2])).slice(0, K).map((p) => p.splice(2) && p);
};
