/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
  var n = prices.length;
  if(n == 0) return 0;
  var stock = new Array(n + 1).fill(0);
  var money = new Array(n + 1).fill(0);
  stock[1] = -prices[0];
  money[1] = 0;
  for(var i = 1; i < n; i++){
      stock[i + 1] = Math.max(stock[i], money[i - 1] - prices[i]);
      money[i + 1] = Math.max(stock[i] + prices[i], money[i]);
  }
  return money[n];
};
