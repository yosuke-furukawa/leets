const path = require('path');
/**
 * @param {string} path
 * @return {string}
 */
var simplifyPath = function(p) {
  return path.resolve(p);
};
