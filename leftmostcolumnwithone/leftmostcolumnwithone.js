/**
 * // This is the BinaryMatrix's API interface.
 * // You should not implement it, or speculate about its implementation
 * function BinaryMatrix() {
 *     @param {integer} row, col
 *     @return {integer}
 *     this.get = function(row, col) {
 *         ...
 *     };
 *
 *     @return {[integer, integer]}
 *     this.dimensions = function() {
 *         ...
 *     };
 * };
 */

/**
 * @param {BinaryMatrix} binaryMatrix
 * @return {number}
 */
var leftMostColumnWithOne = function(binaryMatrix) {
    var [row, col] = binaryMatrix.dimensions();
    var pos = -1;
    var c=col-1;
    var r=0;
    
    while(r<row && c>=0) {
        var m = binaryMatrix.get(r, c);
        if (m === 1) {
            pos = c;
            c = c-1;
        } else {
            r = r+1;
        }    
    }    
    return pos;
};
