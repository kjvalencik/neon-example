'use strict';

const assert = require('assert');

const {
	arrayProcess,
	arrayProcessSerde,
} = require('../native');

const ops = [{
	operator: 'print',
	value: 'Hello, World!'
}, {
	operator: 'print',
	value: 'Hello, Neon!'
}];

arrayProcess(ops);
arrayProcessSerde(ops);
