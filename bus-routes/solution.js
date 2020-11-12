/**
 * @param {number[][]} routes
 * @param {number} S
 * @param {number} T
 * @return {number}
 */
var numBusesToDestination = function(routes, S, T) {
  if (S === T) {
    return 0;
  }
  var map = new Map();
  for (var ri=0;ri<routes.length;ri++) {
    var route = routes[ri];
    for (const p of route) {
      if (map.has(p)) {
        map.get(p).add(ri);
        continue;
      }
      map.set(p, new Set([ri]));
    }
  }
  // console.log(map);
  
  var visited = new Set();
  var queue = [{station:S, hop:1}];
  while (queue.length > 0) {
    var {station, hop} = queue.shift();
    var routeIds = map.get(station);
    if (routeIds == null) {
      continue;
    }
    for (const ri of routeIds) {
      if (visited.has(ri)) {
        continue;
      }
      visited.add(ri);
      var destination = routes[ri];
      if (destination.includes(T)) {
        return hop;
      }
      for (var d of destination) {
        queue.push({ station: d, hop: hop+1 });
      }
    }
  }
  return -1;
};
