const urlMap = new Map();
const keyMap = new Map();
const prefix = "http://tinyurl.com/";
/**
 * Encodes a URL to a shortened URL.
 *
 * @param {string} longUrl
 * @return {string}
 */
var encode = function(longUrl) {
  if (urlMap.has(longUrl)) {
    return urlMap.get(longUrl);
  }
  const key = Math.random().toString(36).slice(-8);
  urlMap.set(longUrl, prefix + key);
  keyMap.set(prefix + key, longUrl);
  return prefix + key;
};

/**
 * Decodes a shortened URL to its original URL.
 *
 * @param {string} shortUrl
 * @return {string}
 */
var decode = function(shortUrl) {
  return keyMap.get(shortUrl);
};

/**
 * Your functions will be called as such:
 * decode(encode(url));
 */
