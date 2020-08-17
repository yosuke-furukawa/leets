/**
 * @param {number} candies
 * @param {number} num_people
 * @return {number[]}
 */
var distributeCandies = function(candies, num_people) {
  var people = new Array(num_people).fill(0);
  var i=1;
  while (candies > 0) {
    var can = candies - i;
    var index = (i-1) % num_people;
    if (can < 0) {
      people[index] += candies;
      break;
    }
    candies = can;
    people[index] += i;
    i++;
  }
  return people;
};
