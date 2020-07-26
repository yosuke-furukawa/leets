/**
 * @param {number[][]} buildings
 * @return {number[][]}
 */
var getSkyline = function(buildings) {
  if (buildings.length === 0) {
    return [];
  }
  if (buildings.length === 1) {
    return [[buildings[0][0], buildings[0][2]], [buildings[0][1], 0]];
  }
  if (buildings.length === 2) {
    return [[buildings[0][0], buildings[0][2]], [buildings[0][1], 0]];
  }
  var xmax = Math.max(...buildings.map(([l,r,h]) => r));
  var ymax = Math.max(...buildings.map(([l,r,h]) => h));
  var matrix = new Array(xmax+2);
  for (var x=0;x<matrix.length;x++) {
    matrix[x] = new Array(ymax+2).fill(0);
  }
  for (var building of buildings) {
    var [l, r, h] = building;
    for (var x=l;x<=r;x++) {
      matrix[x][h] = 1;
    }
    for (var y=0;y<=h;y++) {
      matrix[l][y] = 1;
      matrix[r][y] = 1;
    }
  }
  
  var x=0;
  var y=0;
  var state = "walk";
  var skylines = [];
  while (x <= xmax) {
    if (state === "walk" && matrix[x][y+1] === 1) {
      state = "climb";
    } else if (state === "climb" && matrix[x][y+1] !== 1) {
      skylines.push([x, y]);
      state = "walk";
    } else if (state === "walk" && matrix[x+1][y] !== 1 && y > 0) {
      state = "down";
    } else if (state === "down" && matrix[x+1][y] === 1) {
      skylines.push([x, y]);
      state = "walk";
    } else if (state === "down" && matrix[x][y-1] !== 1) {
      skylines.push([x, y]);
      state = "walk";
    } 
    
    
    if (state === "walk") {
      x++;      
    } else if (state === "climb") {
      y++;
    } else {
      y--;
    }
  }
  return skylines;
};
