/**
 * @param {number[]} prices
 * @param {number} fee
 * @return {number}
 */
var maxProfit = function(prices, fee) {
  var len = prices.length;
  var buy = 0;
  var sell = -prices[0];
  for (var i=1;i<len;i++) {
    buy = Math.max(buy, sell + prices[i] - fee);
    sell = Math.max(sell, buy - prices[i]);
  }
  return buy;
};
