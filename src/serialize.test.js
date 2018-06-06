'use strict';

const assert = require('assert');

const { parse, stringify } = require('../native');

const fixture = {
	a: 1,
	b: {
		c: [2, 3, {
			d: '4'
		}]
	}
};

assert.deepStrictEqual(parse(JSON.stringify(fixture)), fixture);
assert.deepStrictEqual(JSON.parse(stringify(fixture)), fixture);
assert.deepStrictEqual(parse(stringify(fixture)), fixture);
