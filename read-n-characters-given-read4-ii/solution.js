/**
 * Definition for read4()
 * 
 * @param {character[]} buf Destination buffer
 * @return {number} The number of characters read
 * read4 = function(buf4) {
 *     ...
 * };
 */

/**
 * @param {function} read4()
 * @return {function}
 */
var solution = function(read4) {
    /**
     * @param {character[]} buf Destination buffer
     * @param {number} n Number of characters to read
     * @return {number} The number of actual characters read
     */
  var buffer = [];
    return function(buf, n) {
      if (buffer.length > 0) {
        buf.push(...buffer.splice(0, n));
      }
      var want = n;
      while (buf.length < n) {
        var b = [];
        read4(b);
        if (b.length === 0) {
          // EOF
          break;
        }
        want = n - buf.length;
        buffer.push(...b.slice(want));
        buf.push(...b.slice(0, want));
      }
     
    };
};
