---
title: JS Tricks
date: 2019-04-14
type: technology
spoiler: JavaScript methods
category: js
tags: [js, tricks]
---

## Matrix

<!-- ```js{5,7-12} -->
```js
/**
 * matrixMultiply
 * @param {array} MatrixA - 2D-array
 * @param {array} MatrixB - 2D-array
 */
function matrixMultiply(MatrixA, MatrixB) {
  const result = new Array(MatrixA.length).fill(0)
    .map(_row => new Array(MatrixB[0].length).fill(0));

  return result.map((row, i) => {
    return row.map((_val, j) => {
      return MatrixA[i].reduce((sum, elm, k) => sum + (elm * MatrixB[k][j]), 0);
    });
  });
}

/**
 * matrixTranspose
 * @param {array} matrix - 2D-array
 * @example
 * => matrixTranspose([[1, 2, 3], [4, 5, 6]])
 * => result: [[1, 4], [2, 5], [3, 6]]
 */
const matrixTranspose = matrix => matrix[0]
  .map((_, i) => matrix.map(row => row[i]));
```

## Object

```js
/**
 * Returns a function that can be called with an object. The return value of the
 * new function is a copy of the object excluding the key passed initially.
 */
const omit = key => object => {
  const { [key]: toRemove, ...rest } = object;
  return rest;
}
```

## Array

```js
// To getting unique values from an array
Array.prototype.unique = function() {
  return this.filter((value, index, self) => (
    self.indexOf(value) === index
  ));
}

// Get a random value from the array
Array.prototype.randomElement = function () {
  return this[Math.floor(Math.random() * this.length)];
};

function arrayMove(arr, oldIndex, newIndex) {
  while (oldIndex < 0) {
    oldIndex += arr.length;
  }
  while (newIndex < 0) {
    newIndex += arr.length;
  }
  if (newIndex >= arr.length) {
    let k = newIndex - arr.length;
    while ((k--) + 1) {
      arr.push(undefined);
    }
  }
  arr.splice(newIndex, 0, arr.splice(oldIndex, 1)[0]);
  return arr;
}

function arrayDeleteItem(array, record) {
  if (Array.isArray(array)) {
    const index = array.indexOf(record);
    if (index !== -1) {
      array.splice(index, 1);
    }
    return array;
  }
  return console.error('arrayDeleteItem(array: T[], record: T)');
};
```

## Random

```js
const randomStr = () => Math.random().toString(36).substring(2);

export function randomStr2(length) {
  let result = '';
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * characters.length));
  }
  return result;
}

const randomDate = () => `${new Date(+(new Date()) - Math.floor(Math.random() * 10000000000))}`;
```

## Format

```js
function formatBytes(bytes, decimals) {
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  const dm = decimals < 0 || !decimals ? 0 : decimals || 2;
  let l = 0;
  let n = parseInt(bytes, 10) || 0;
  while (n >= 1024 && ++l) {
    n = n / 1024;
  }
  return(n.toFixed(dm) + units[l]);
};

Date.prototype.format = function(fmt) {
  const o = {
    'Y+': this.getFullYear(),
    'M+': this.getMonth() + 1,
    'D+': this.getDate(),
    'h+': this.getHours(),
    'm+': this.getMinutes(),
    's+': this.getSeconds(),
    'q+': Math.floor((this.getMonth() + 3) / 3),
    'S+': this.getMilliseconds(),
  };
  for (const k in o) {
    if (new RegExp(`(${k})`).test(fmt)) {
      if (k === 'Y+') {
        fmt = fmt.replace(RegExp.$1, (`${o[k]}`).substr(4 - RegExp.$1.length));
      } else if (k === 'S+') {
        let lens = RegExp.$1.length;
        lens = lens === 1 ? 3 : lens;
        fmt = fmt.replace(RegExp.$1, (`00${o[k]}`).substr((`${o[k]}`).length - 1, lens));
      } else {
        fmt = fmt.replace(RegExp.$1, (RegExp.$1.length === 1) ? (o[k]) : ((`00${o[k]}`).substr((`${o[k]}`).length)));
      }
    }
  }
  return fmt;
};

// timestamp
const timestamp = time => Date.parse(time) / 1000

/// numberFormat
const numberFormat = num => (/^\d+$/.test(num) ? `${num}`.replace(/\B(?=(\d{3})+$)/g, ',') : '--');
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString
const numberFormat2 = (num) => num.toLocaleString('en-US');

// currencyFormat
// https://www.currency-iso.org/en/home/tables/table-a1.html
// currency: USD, EUR, CNY, ...
const currencyFormat = (num, currency = 'USD') => num.toLocaleString('en-US', {style: 'currency', currency});
```

```js
// https://stackoverflow.com/questions/18017869/build-tree-array-from-flat-array-in-javascript
/**
const list = [
  { text: 'a', id: 1, parentId: 0 },
  { text: 'a-x', id: 2, parentId: 1 },
  { text: 'a-y', id: 3, parentId: 1 },
  { text: 'a-x-1', id: 4, parentId: 2 },
  { text: 'a-x-2', id: 5, parentId: 2 },
  // ...
]
*/
function listToTree(list, parentId = 'parentId') {
  let map = {}, node, roots = [], i;
  for (i = 0; i < list.length; i += 1) {
    map[list[i].id] = i; // initialize the map
    list[i].children = []; // initialize the children
  }
  for (i = 0; i < list.length; i += 1) {
      node = list[i];
      if (node[parentId] !== 0) {
          // if you have dangling branches check that map[node[parentId]] exists
          list[map[node[parentId]]].children.push(node);
      } else {
          roots.push(node);
      }
  }
  return roots;
}
```

<!-- ## URL

```js
const getParam = key => new URLSearchParams(window.location.search.substring(1)).get(key) || ''

// genParams({a: 1, b: 2})
// output: a=1&b=2
const genParams = obj => Object.entries(obj).map(([key, val]) => `${key}=${val}`).join('&');
``` -->

## Cookie

```js
// cookie.js
export default {
  set(name, value, days, path) {
    let expires = '';
    if (days) {
      const date = new Date();
      date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
      expires = `; expires=${date.toGMTString()}`;
    }

    const dir = path || '/';
    document.cookie = `${name}=${value}${expires}; path=${dir}`;
  },
  get(name) {
    const nameEQ = `${name}=`;
    const ca = document.cookie.split(';');
    for (let i = 0; i < ca.length; i += 1) {
      let c = ca[i];
      while (c.charAt(0) === ' ') {
        c = c.substring(1, c.length);
      }
      if (c.indexOf(nameEQ) === 0) {
        return c.substring(nameEQ.length, c.length);
      }
    }
    return '';
  },
  remove(name) {
    this.set(name, '', -1);
  },
};
```

## URL

```js
// query.js
export default {
  parse(search = window.location.search) {
    if (!search) return;
    const hashes = search.slice(search.indexOf('?') + 1).split('&');
    return hashes.reduce((acc, hash) => {
      const [key, val] = hash.split('=');
      return { ...acc, [key]: decodeURIComponent(val) };
    }, {});
  },

  get(key) {
    const params = this.parse(window.location.search);
    if (params) {
      return params[key];
    }
  },

  stringify(query) {
    const str = Object.keys(query)
      .map(key => `${key}=${encodeURIComponent(query[key] || '')}`)
      .join('&');
    return str;
  },
};
```

### URL Extract

```js
// url-extract.js
export const matchURL = /((https?|http|ftp|file):\/\/[-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|])/g;
export const textURL = (str, text) => str.replace(matchURL, `<a href="$1">${text || '$1'}</a>`);
```

### Algorithm

#### Fibonacci sequence

```js
function fib(n) {
  if (n == 1 || n == 2) return 1;
  return fib(n - 1) + fib(n - 2);
}

function fib2(n) {
  let dp = [0];
  dp[1] = dp[2] = 1;
  for (let i = 3; i <= n; i++) {
    dp[i] = dp[i - 1] + dp[i - 2];
  }
  return dp[n];
}

function fib3(n) {
  if (n === 2 || n === 1) return 1;
  let prev = 1, curr = 1;
  for (let i = 3; i <= n; i++) {
    const sum = prev + curr;
    prev = curr;
    curr = sum;
  }
  return curr;
}s
```