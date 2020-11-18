/**
 * @param {number[][]} grid
 * @return {number}
 */
var cherryPickup = function(grid) {
  let n = grid.length, steps = 2*n-1;
  let dp = Array.from({length:n}, x=>Array.from({length:n}, y=>0));
  dp[0][0]=grid[0][0];
  for(let k=1; k<steps; k++){
    //notice that we iterate from n-1 to 0 because we want to reuse the results in dp we calculated the last iteration. see my graph below to understand it.
    for(let i=n-1; i>=0; i--){
      for(let x=n-1; x>=0; x--){
        let j=k-i, y=k-x;
        if(j<0 || y<0 || j>n-1 || y>n-1 || grid[i][j]===-1 || grid[x][y]===-1){
          dp[i][x]=-1;
          continue;
        }

        if(i>0) dp[i][x]=Math.max(dp[i][x], dp[i-1][x]);
        if(x>0) dp[i][x]=Math.max(dp[i][x], dp[i][x-1]);
        //notice that dp[i][x] does not mean grid[i][x], I myself got confused before here
        if(i>0 && x>0) dp[i][x]=Math.max(dp[i][x], dp[i-1][x-1]);

        //check if the paths of a and b cross at grid[i][j]
        if(dp[i][x]>=0) dp[i][x]+= grid[i][j]+(i===x?0:grid[x][y]);
      }
    }
  }
  return Math.max(dp[n-1][n-1],0);
};
