/**
 * // This is the HtmlParser's API interface.
 * // You should not implement it, or speculate about its implementation
 * function HtmlParser() {
 *
 *		@param {string} url
 *     	@return {string[]}
 *     	this.getUrls = function(url) {
 *      	...
 *     	};
 * };
 */


/**
 * @param {string} startUrl
 * @param {HtmlParser} htmlParser
 * @return {string[]}
*/
var crawl = function(startUrl, htmlParser) {
  const { hostname } = new URL(startUrl);
  const visited = new Set();
  visited.add(startUrl);
  const queue = [startUrl];
  
  while (queue.length > 0) {
    const url = queue.shift();
    const urls = htmlParser.getUrls(url);
    for (const u of urls) {
      if (visited.has(u)) {
        continue;    
      }
      const urlObject = new URL(u);
      if (hostname === urlObject.hostname) {
        visited.add(u);
        queue.push(u);
      }
    }
  }
  return [...visited];
};
