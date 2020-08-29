/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
var findDiagonalOrder = function(matrix) {
  function reverse() {
        x = -x;
        y = -y;
    }

    if (matrix.length < 1) {
        return [];
    }

    let res = [], row = 0, col = 0, x = 1, y = -1;
    while (res.length < matrix.length * matrix[0].length) {
        res.push(matrix[col][row]);
        row += x;
        col += y;
        if (col < 0 && row < matrix[0].length) {
            reverse();
            col = 0;
        }
        if (row < 0 && col < matrix.length) {
            reverse();
            row = 0;
        }
        if (row >= matrix[0].length) {
            reverse();
            col += 2 * y;
            --row;
        }
        if (col >= matrix.length) {
            reverse();
            row += 2 * x;
            --col;
        }
    }
    return res;
};
