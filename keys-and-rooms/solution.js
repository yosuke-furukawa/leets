/**
 * @param {number[][]} rooms
 * @return {boolean}
 */
var canVisitAllRooms = function(rooms) {
  var queue = [...rooms[0]];
  rooms[0] = [];
  
  while (queue.length > 0) {
    const key = queue.shift();
    if (rooms[key]?.length > 0) {
      queue.push(...rooms[key].splice(0, rooms[key].length));
    }
  }
  
  return rooms.filter((a) => a.length > 0).length === 0;
};
