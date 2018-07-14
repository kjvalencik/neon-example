'use strict';

const assert = require('assert');
const util = require('util');

const {
	parse,
	stringify,
	performAsyncTask: performAsyncTaskCB
} = require('../native');

const performAsyncTask = util.promisify(performAsyncTaskCB);

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

performAsyncTask().then(res => assert.strictEqual(res, 17));
