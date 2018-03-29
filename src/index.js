'use strict';

const Server = require('./server');

const server = Server();

server
	.listen()
	.then(() => console.log(`Listening on ${server.port}`));
