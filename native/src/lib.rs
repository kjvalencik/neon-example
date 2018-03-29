#[macro_use]
extern crate neon;
extern crate neon_serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_json;

use neon::js::JsValue;
use neon::js::error::{JsError, Kind};
use neon::vm::{Call, JsResult};

use serde_bytes::ByteBuf;

#[derive(Deserialize)]
struct Request {
	body: ByteBuf,
}

#[derive(Deserialize)]
struct HelloRequest {
	name: String,
}

#[derive(Serialize)]
struct HelloResponse {
	greeting: String,
}

fn hello(call: Call) -> JsResult<JsValue> {
	let scope = call.scope;
    let arg0 = call.arguments
         .require(scope, 0)?
         .check::<JsValue>()?;

	let req: Request = neon_serde::from_value(scope, arg0)?;
	let req_body: HelloRequest = serde_json::from_slice(&req.body)
		.or_else(|err| JsError::throw(Kind::Error, &err.to_string()))?;

	let res = HelloResponse {
		greeting: format!("Hello, {}!", req_body.name),
	};

	let res_body = serde_json::to_vec(&res)
		.or_else(|err| JsError::throw(Kind::Error, &err.to_string()))?;

	let ret = neon_serde::to_value(scope, &ByteBuf::from(res_body))?;

    Ok(ret)
}

register_module!(m, {
    m.export("hello", hello)
});
