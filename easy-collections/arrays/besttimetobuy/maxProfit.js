var maxProfit = function(prices) {
  let result = 0;
  let buy = prices[0], sell = 0;
  for (let i=0; i<prices.length; i++) {
    if (prices[i] < prices[i+1]) {
      buy = prices[i];
    } else {
      continue;
    }
    for (let j=i+1; j<prices.length; j++) {
      if (prices[j] > prices[j+1]) {
        sell = prices[j];
        if (sell - buy > 0) {
          result += sell - buy;
          i = j;
          buy = prices[i+1];
          sell = 0;
          break;
        }
      }

      if (j === prices.length - 1) {
        result += prices[j] - buy;
        i = j;
      }
    }
  }
  return result;
};
