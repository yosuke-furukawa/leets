var rotate = function(matrix) {
  var width = matrix[0].length-1;
  var height = matrix.length-1;
  // reverse x => width;
  for (var y=0;y<=height;y++) {
    matrix[y].reverse();
  }

  var map = new Map();
  for (var y=0;y<=height;y++) {
    for (var x=0;x<=width;x++) {
      var nx = (height - y);
      var ny = (width - x);
      if (map.has(`${x}, ${y}`) || map.has(`${nx}, ${ny}`)) {
        continue;
      }
      [matrix[ny][nx],  matrix[y][x]] = [matrix[y][x],  matrix[ny][nx]];
      map.set(`${x}, ${y}`, `${nx}, ${ny}`);
      map.set(`${nx}, ${ny}`, `${x}, ${y}`);
    }
  }
  return matrix;
};

console.log(rotate(
[
  [1,2,3],
  [4,5,6],
  [7,8,9]
]
));

console.log(rotate(
[
  [ 5, 1, 9,11],
  [ 2, 4, 8,10],
  [13, 3, 6, 7],
  [15,14,12,16]
]
));
