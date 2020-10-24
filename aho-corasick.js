class State {
  constructor(id) {
    this.id = id;
    this.next = {};
  }

  key(x) {
    return this.next[x];
  }
}
class AhoCorasick {
  constructor(words) {
    this.state = [ new State(0) ];
    this.output = [[]];
    this.makeGoto(words);
    this.makeFailure();
  }

  makeGoto(words) {
    for (const word of words) {
      let curr = this.state[0];
      for (const c of [...word]) {
        if (curr.key(c) == null) {
          const next = new State(this.state.length);
          curr.next[c] = next;

          this.state.push(next);
          this.output.push([]);
        }
        curr = curr.next[c];
      }
      const id = curr.id;
      this.output[id].push(word);
    }
  }

  makeFailure() {
    const failure = new Array(this.state.length).fill(0);
    const queue = [ 0 ];
    while (queue.length > 0) {
      const s = queue.shift();
      for (const c of Object.keys(this.state[s].next)) {
        const next = this.g(s, c);

        if (next != null) {
          queue.push(next);
        }

        if (s != 0) {
          let f = failure[s];
          while (this.g(f, c) == null) {
            f = failure[f];
          }
          failure[next] = this.g(f, c);

          this.output[next].concat(this.output[failure[next]]);
        }
      }
    }
    this.failure = failure;
  }

  g(s, c) {
    if (c in this.state[s].next) {
      return this.state[s].next[c].id;
    } else {
      if (s === 0) {
        return 0;
      } else {
        return;
      }
    }
  }

  match(query) {
    let s = 0;
    for (var i=0;i<query.length;i++) {
      while (this.g(s, query[i]) == null) {
        s = this.failure[s];
      } 

      s = this.g(s, query[i]);

      for (const c of this.output[s]) {
        console.log('%d,%d => %s', i - c.length + 1, i, c);
      }
    }
  }
}


const ac = new AhoCorasick(['ab', 'bc', 'bab', 'd', 'abcde']);
ac.match('xbabcdex');

