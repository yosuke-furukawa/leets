/**
 * @param {number[][]} points
 * @return {number}
 */
var findMinArrowShots = function(points) {
  const balloons = points.slice().sort((a, b) => a[1] - b[1]);
  let shots = 0;

  let target = balloons[0][1];
  let done = false
  for (let i = 0; i < balloons.length; i++) {
    if (balloons[i][0] > target && balloons[i][1] > target) {
      shots += 1;
      target = balloons[i][1]
    }
  }
  
  return shots + 1
};
