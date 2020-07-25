/**
 * @param {number[][]} people
 * @return {number[][]}
 */
var reconstructQueue = function(people) {
  var result = new Array(people.length).fill([Infinity,Infinity]);
  people.sort(([a1, b1], [a2, b2]) => a1-a2 || b1-b2);
  while(people.length > 0) {
    var person = people.shift();
    var position = person[1];
    var pos = 0;
    var highernum = 0;
    for (var r of result) {
      if (position === highernum) {
        while (result[pos][0] !== Infinity) {
          pos++;
        }
        break;
      }
      if (r[0] >= person[0]) {
        highernum++;
      }
      pos++;
    }
    result.splice(pos, 1, person);
  }
  return result;
};
