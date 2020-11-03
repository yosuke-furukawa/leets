/**
 * @param {string[][]} tickets
 * @return {string[]}
 */
var findItinerary = function(tickets) {
    if (tickets.length === 0) {
    return [];
  }
  tickets = tickets.sort((a, b) => a[1].localeCompare(b[1]));
  var map = new Map();
  for (var ticket of tickets) {
    if (map.has(ticket[0])) {
      map.get(ticket[0]).push(ticket[1]);
    } else {
      map.set(ticket[0], [ticket[1]]);
    }
  }
  var result = [];
  var backtrack = (result, map, current) => {
    result.push(current);
    if (result.length === tickets.length + 1) {
      return true;
    }

    if (!map.has(current) || map.get(current).length === 0) {
      return false;
    }

    var arrivals = map.get(current);
    for (var i=0;i<arrivals.length;i++) {
      var [arrival] = map.get(current).splice(i, 1);
      if (backtrack(result, map, arrival)) {
        return true;
      }

      result.splice(result.length - 1, 1);
      arrivals.splice(i, 0, arrival);
    }
  };
  backtrack(result, map, "JFK");
  return result;
};
