#[macro_use]
extern crate neon;
extern crate neon_serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_json;

use neon::js::error::{JsError, Kind};
use neon::js::{Object, JsArray, JsFunction, JsNumber, JsObject, JsString, JsUndefined, JsValue};
use neon::scope::Scope;
use neon::task::Task;
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

fn parse(call: Call) -> JsResult<JsValue> {
	let scope = call.scope;
	let s = call.arguments
		.require(scope, 0)?
		.check::<JsString>()?;

	let o: serde_json::Value = serde_json::from_str(&s.value())
		.or_else(|err| JsError::throw(Kind::Error, &err.to_string()))?;

	let o = neon_serde::to_value(scope, &o)?;

	Ok(o)
}

fn stringify(call: Call) -> JsResult<JsString> {
	let scope = call.scope;
	let o = call.arguments
		.require(scope, 0)?
		.check::<JsValue>()?;

	let o: serde_json::Value = neon_serde::from_value(scope, o)?;
	let s = serde_json::to_string(&o)
		.or_else(|err| JsError::throw(Kind::Error, &err.to_string()))?;

	JsString::new_or_throw(scope, &s)
}

struct SuccessTask;

impl Task for SuccessTask {
    type Output = i32;
    type Error = String;
    type JsEvent = JsNumber;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        Ok(17)
    }

    fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        Ok(JsNumber::new(scope, result.unwrap() as f64))
    }
}

fn perform_async_task(call: Call) -> JsResult<JsUndefined> {
    let f = call.arguments.require(call.scope, 0)?.check::<JsFunction>()?;
    SuccessTask.schedule(f);
    Ok(JsUndefined::new())
}


fn array_process(call: Call) -> JsResult<JsUndefined> {
	let scope = call.scope;
	let arr = call.arguments.require(scope, 0)?.check::<JsArray>()?;

	for i in 0..arr.len() {
		let item = arr.get(scope, i)?.check::<JsObject>()?;
		let operator = item.get(scope, "operator")?.check::<JsString>()?.value();
		let value = item.get(scope, "value")?.check::<JsString>()?.value();

		match operator.as_str() {
			"print" => {
				println!("{}", value);
			},
			_ => {
				let msg = format!("Unsupported operator: {}", operator);

				return JsError::throw(Kind::Error, &msg);
			}
		}
    }

	Ok(JsUndefined::new())
}

#[derive( Deserialize)]
#[serde(tag = "operator")]
enum Operation {
	#[serde(rename = "print")]
	Print { value: String }
}

fn array_process_serde(call: Call) -> JsResult<JsUndefined> {
	let scope = call.scope;
	let arg0 = call.arguments.require(scope, 0)?;
	let ops: Vec<Operation> = neon_serde::from_value(scope, arg0)?;

	for op in ops {
		match op {
			Operation::Print { value } => {
				println!("{}", value);
			}
		}
	}

	Ok(JsUndefined::new())
}

register_module!(m, {
	m.export("parse", parse)?;
	m.export("stringify", stringify)?;
	m.export("hello", hello)?;
	m.export("performAsyncTask", perform_async_task)?;
	m.export("arrayProcess", array_process)?;
	m.export("arrayProcessSerde", array_process_serde)?;

	Ok(())
});
