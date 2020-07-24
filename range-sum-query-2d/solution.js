

/**
 * @param {number[][]} matrix
 */
var NumMatrix = function(matrix) {
  this.matrix = matrix;
  this.ft = new FenwickTree(matrix);
};

/** 
 * @param {number} row 
 * @param {number} col 
 * @param {number} val
 * @return {void}
 */
NumMatrix.prototype.update = function(row, col, val) {
  this.ft.update(row+1, col+1, val-this.matrix[row][col]);
  this.matrix[row][col] = val;
};

/** 
 * @param {number} row1 
 * @param {number} col1 
 * @param {number} row2 
 * @param {number} col2
 * @return {number}
 */
NumMatrix.prototype.sumRegion = function(row1, col1, row2, col2) {
  return this.ft.query2d(row1, col1, row2, col2);
};

/** 
 * Your NumMatrix object will be instantiated and called as such:
 * var obj = new NumMatrix(matrix)
 * obj.update(row,col,val)
 * var param_2 = obj.sumRegion(row1,col1,row2,col2)
 */

/**
 * FenwickTree
 */
class FenwickTree {
  constructor(matrix) {
    var n = matrix.length;
    if (matrix.length === 0) {
      this.ft = [[]];
      return;
    }
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
    return x & (-x);
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
      for(var yi=y;yi < this.ft[0].length; yi = yi + this.lsb(yi)) {
        this.ft[xi][yi] += value;
      }
    }
  }
}

