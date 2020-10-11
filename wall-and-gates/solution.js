/**
 * @param {number[][]} rooms
 * @return {void} Do not return anything, modify rooms in-place instead.
 */
var wallsAndGates = function(rooms) {
  const queue = [];
  
  for (var x=0;x<rooms.length;x++) {
    for (var y=0;y<rooms[x].length;y++) {
      if (rooms[x][y] === 0) {
        queue.push([x, y, 0]);
      }
    }
  }
  
  while (queue.length > 0) {
    const [x, y, v] = queue.pop();
    
    if (rooms[x-1]?.[y] === 2147483647) {
      rooms[x-1][y] = v+1;
      queue.unshift([x-1, y, v+1]);
    }
    if (rooms[x+1]?.[y] === 2147483647) {
      rooms[x+1][y] = v+1;
      queue.unshift([x+1, y, v+1]);
    }
    if (rooms[x]?.[y-1] === 2147483647) {
      rooms[x][y-1] = v+1;
      queue.unshift([x, y-1, v+1]);
    }
    if (rooms[x]?.[y+1] === 2147483647) {
      rooms[x][y+1] = v+1;
      queue.unshift([x, y+1, v+1]);
    }
    
  }
};
