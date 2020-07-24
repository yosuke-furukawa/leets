class FenwickTree {
  constructor(matrix) {
    var n = matrix.length;
    var m = matrix[0].length;
    this.ft = new Array(n+1);
    for (var row=0;row<n+1;row++) {
      this.ft[row] = new Array(m+1).fill(0);
    }
    for (var i=0;i<n;i++) {
      for (var j=0;j<m;j++) {
        this.update(i+1, j+1, matrix[i][j]);
      }
    }
  }

  lsb(x) {
    var l =  x & (-x);
    if (l === 0) {
      return 1;
    }
    return l;
  }

  query(x, y) {
    var sum = 0;
    for (var xi = x; xi > 0; xi = xi - this.lsb(xi)) {
      for (var yi = y; yi > 0; yi = yi - this.lsb(yi)) {
        sum = sum + this.ft[xi][yi];
      }
    } 
    return sum;
  }

  query2d(x1, y1, x2, y2) {
    var res1 = this.query(x2+1, y2+1);
    var res2 = this.query(x1, y2+1);
    var res3 = this.query(x2+1, y1);
    var res4 = this.query(x1, y1);
    return res1 - res2 - res3 + res4;
  }

  update(x, y, value) {
    for(var xi=x;xi < this.ft.length; xi = xi + this.lsb(xi)) {
      for(var yi=y;yi < this.ft[xi].length; yi = yi + this.lsb(yi)) {
        this.ft[xi][yi] += value;
      }
      
    }
  }
}

function main() {
  var ft = new FenwickTree([
    [3,0,1,4,2],[5,6,3,2,1],[1,2,0,1,5],[4,1,0,1,7],[1,0,3,0,5]
  ]);

  console.log(ft.query2d(2,1,4,3));
  ft.update(3,2,2);
  console.log(ft.query2d(2,1,4,3));

  ft = new FenwickTree([
    [1]
  ]);

  console.log(ft.query2d(0,0,0,0));
  ft.update(0,0,-1);
  console.log(ft.query2d(0,0,0,0));
}

main();
