/**
 * Definition for read4()
 * 
 * @param {character[]} buf4 Destination buffer
 * @return {number} The number of actual characters read
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
    return function(buf, n) {
      if (n === 0) {
        return 0;
      }
      var bytes = read4(buf);
      if (bytes < 4) {
        if (bytes > n) {
          buf.splice(n, buf.length);
        }
        return bytes;
      }
      
      while(bytes <= n) {
        var a = [];
        var b = read4(a);
        buf.push(...a);

        bytes += b;
        if (b === 0) {
          break;
        }
      }
      var ans = bytes;
      if (bytes > n) {
        ans = n;
        buf.splice(n, buf.length);
      }
      return bytes > n ? n : bytes;
    };
};
