/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
  var max = prices[0];
  var min = prices[0];
  var result = 0;
  for (var i=1;i<prices.length;i++) {
      if (min > prices[i]) {
          min = prices[i];
          max = min;
      } else if (max < prices[i]) {
          max = prices[i];
          if (result < (max - min)) {
            result = max - min;              
          }
      }
  }
  return result;
};
