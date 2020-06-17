/**
 * @param {character[][]} matrix
 * @return {number}
 */
var maximalSquare = function(matrix) {
    var dp = new Array(matrix.length);
    for (var i=0;i<matrix.length;i++) {
        if (!dp[i]) {
            dp[i] = [];
        }
        for (var j=0;j<matrix[i].length;j++) {
            dp[i][j] = +matrix[i][j];
        }
    }
    var max = 0;
    for (var i=0;i<dp.length;i++) {
        for (var j=0;j<dp[i].length;j++) {
            if (dp[i][j] === 1) {
                if (i-1 < 0 || j-1 < 0) {
                  max = max < dp[i][j] ? dp[i][j] : max; 
                  continue;
                }
                dp[i][j] = Math.min(dp[i][j-1], dp[i-1][j], dp[i-1][j-1]) + 1;
                if (max < dp[i][j]) {
                    max = dp[i][j];
                }
            }
        }
    }
    return max**2;
};
