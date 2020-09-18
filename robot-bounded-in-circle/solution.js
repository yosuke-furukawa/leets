/**
 * @param {string} instructions
 * @return {boolean}
 */
var isRobotBounded = function(instructions) {
  var direction = 0;
  var positionX = 0;
  var positionY = 0;
  for (var i=0;i<instructions.length;i++) {
    var inst = instructions[i];
    if (inst === "L") {
      direction++;
    } else if (inst === "R") {
      direction--;
    } else if (inst === "G") {
      var dir = direction - 4 * Math.floor(direction / 4);
      if (dir === 1) {
        positionX++;
      } else if (dir === 2) {
        positionY--;
      } else if (dir === 3) {
        positionX--;
      } else {
        positionY++;
      }
    }
  }
  // console.log(positionX, positionY, direction);
  if (positionX === 0 && positionY === 0) {
    return true;
  }
  var dir = direction - 4 * Math.floor(direction / 4);
  if (dir === 0) {
    return false;
  }
  return true;
};
