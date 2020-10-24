/**
 * @param {number[]} tokens
 * @param {number} P
 * @return {number}
 */
var bagOfTokensScore = function(tokens, P) {
  var sorted = tokens.sort((a, b) => a-b);
  var p = P;
  var score = 0;
  while (sorted.length > 0) {
    if (sorted[0] <= p) {
      const s = sorted.shift();
      p = p-s;
      score++;
      continue;
    }
    if (score > 0 && sorted.length > 1 && sorted[sorted.length - 1] + p > sorted[0]) {
      const s = sorted.pop();
      p = p + s;
      score--;
      continue;
    } else {
      break;
    }
  }
  return score;
};
