/**
 * @param {string} S
 * @param {number} K
 * @return {string}
 */
var decodeAtIndex = function(S, K) {
  let cnt = 0;
  let i = 0;
  for (; i < S.length; ++i) {
    const ch = S[i];
    if (ch != +ch) {
      cnt++;
    } else {
      cnt *= +ch;
    }
    if (cnt >= K) {
      break;
    }
  }
  for (let j = i; j >= 0; --j) {
    const ch = S[j];
    if (ch != +ch) {
      if (K === cnt) return S[j];
      cnt--;
    } else {
      cnt /= +ch;
      K %= cnt;
      if (K === 0) K = cnt;
    }
  }
};
