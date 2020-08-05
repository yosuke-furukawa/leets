/**
 * @param {number[]} ratings
 * @return {number}
 */
var candy = function(ratings) {
  if (ratings.length === 1) {
    return 1;
  }
  var prev = -1;
  var next;
  var candies = new Array(ratings.length).fill(1);
  var ratingsOrder = ratings.map((rating, i) => ({ rate: rating, index: i}));
  ratingsOrder = ratingsOrder.sort((a, b) => a.rate - b.rate);
  while (ratingsOrder.length > 0) {
    var rating = ratingsOrder.shift();
    var {rate, index} = rating;
    var prev = index - 1;
    var next = index + 1;
    var prevRate = ratings[prev] ?? Infinity;
    var nextRate = ratings[next] ?? Infinity;
    var prevCandy = candies[prev] ?? 0;
    var nextCandy = candies[next] ?? 0;
    
    if (rate > prevRate || rate > nextRate) {
      if (rate > prevRate && rate > nextRate) {
        candies[index] = Math.max(prevCandy, nextCandy)+1;
      } else if (prevRate < nextRate) {
        candies[index] = prevCandy + 1;
      } else if (prevRate > nextRate) {
        candies[index] = nextCandy + 1;
      }
    }
  }

  return candies.reduce((acc, cur) => acc+cur);
};
