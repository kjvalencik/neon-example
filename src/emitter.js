'use strict';

const { EventEmitter } = require('events');
const { promisify } = require('util');

const { EventEmitter: RustChannel } = require('../native');

class MyEventEmitter extends EventEmitter {
	constructor() {
		super();

		const channel = new RustChannel();
		const poll = promisify(channel.poll.bind(channel));

		this._shutdown = false;

		const loop = () => {
			if (this._shutdown) {
				return channel.shutdown();
			}

			poll()
				.then(({ event, ...data }) => this.emit(event, data))
				.catch(err => this.emit('error', err))
				.then(() => setImmediate(loop));
		};

		loop();
	}

	shutdown() {
		this._shutdown = true;

		return this;
	}
}

function run() {
	const emitter = new MyEventEmitter();

	emitter.on('tick', ({ count }) => console.log(count));
	setTimeout(() => emitter.shutdown(), 5000);
}

run();
