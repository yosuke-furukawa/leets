/**
 * @param {number} num
 * @return {number[]}
 */
var countBits = function(num) {
  var dp = new Array(num+1).fill(0);
  var n = 1;
  for (var i=1;i<num+1;i++) {
    if (i % 2 === 1) {
      dp[i] = dp[i-1] + 1;
    } else if (i === 2 ** n) {
      n++;
      dp[i] = 1;
    } else {
      dp[i] = numofbits(i);
    }
  }
  return dp;
};

function numofbits(bits) {
    bits = (bits & 0x55555555) + (bits >> 1 & 0x55555555);
    bits = (bits & 0x33333333) + (bits >> 2 & 0x33333333);
    bits = (bits & 0x0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f);
    bits = (bits & 0x00ff00ff) + (bits >> 8 & 0x00ff00ff);
    return (bits & 0x0000ffff) + (bits >>16 & 0x0000ffff);
}
