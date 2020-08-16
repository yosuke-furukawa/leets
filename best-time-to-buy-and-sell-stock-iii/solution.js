/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
  if(prices == null || prices.length < 2) return 0;

  var minLR = prices[0];
  var maxRL = prices[prices.length - 1];
  var dpLR = new Array(prices.length).fill(0);
  var dpRL = new Array(prices.length).fill(0);
  var max = 0;

  for(var i = 1; i < prices.length; i++){
    if(prices[i] < minLR) minLR = prices[i];
    dpLR[i] = Math.max(dpLR[i - 1], prices[i] - minLR);
  }

  for(var i = prices.length - 2; i >= 0; i--){
    if(prices[i] > maxRL) maxRL = prices[i];
    dpRL[i] = Math.max(dpRL[i + 1], maxRL - prices[i]);
  }

  for(var i = 0; i < prices.length; i++){
    max = Math.max(max, dpLR[i] + dpRL[i]);
  }

  return max;
};
