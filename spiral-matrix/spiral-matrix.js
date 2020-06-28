/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
var spiralOrder = function(matrix) {
  if (matrix.length === 0) {
    return [];
  }
  var col = 0;
  var row = 0;
  var beginCol = 1;
  var lastCol = matrix.length-1;

  var beginRow = 0;
  var lastRow = matrix[0].length-1;
  var direction = lastRow > 0 ? 0 : 1; // horizon: 0, vertical:1, antihorizon: 2, antivertical: 3
  var result = [];
  var precol;
  var prerow;
  while(true) {
    console.log({col, row, beginCol, lastCol, beginRow, lastRow});

    if (precol === col && prerow === row) {
      break;
    }
    result.push(matrix[col][row]);
    precol = col;
    prerow = row;
    console.log(result);    
    if (direction === 0) {
      if (row < lastRow) {
        row++;
      }
      if (row === lastRow) {
        direction = 1; // horizon to vertical
        lastRow--;
        continue;
      }
      if (row > lastRow) {
        break;
      }
    } else if (direction === 1) {
      if (col < lastCol) {
        col++;
      }
      if (col === lastCol) {
        direction = 2; // vertical to antihorizon
        lastCol--;
        continue;
      }
      if (col > lastCol) {
        break;
      }
    } else if (direction === 2) {
      if (row > beginRow) {
        row--;
      }
      if (row === beginRow) {
        direction = 3; // vertical to antihorizon
        beginRow++;
        continue;
      }
      if (row < beginRow) {
        break;
      }
    } else if (direction === 3) {
      if (col > beginCol) {
        col--;
      }
      if (col === beginCol) {
        direction = 0; // vertical to antihorizon
        beginCol++;
        continue;
      }
      if (col < beginCol) {
        break;
      }
    }
  }
  return result;
};
