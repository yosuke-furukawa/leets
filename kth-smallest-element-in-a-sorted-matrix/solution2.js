var kthSmallest = function(matrix, k) {
  let n = matrix.length;
  let lo = matrix[0][0];
  let hi = matrix[n-1][n-1];

  while(lo < hi){
    let count = 0;
    let mid = Math.floor((hi+lo)/2);
    console.log(mid)
    for (let i=0;i<n;i++){
      for (let j=0;j<n;j++){
        if (matrix[i][j] <= mid) count++;
        else break;
      }
    }
    if (count < k) lo = mid + 1;
    else if (count >= k)   hi = mid;
  }
  return lo
};
