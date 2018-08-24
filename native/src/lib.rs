#[macro_use]
extern crate neon;
extern crate neon_serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_json;

use neon::context::{Context, FunctionContext, TaskContext};
use neon::object::Object;
use neon::result::{JsResult, JsResultExt};
use neon::task::Task;
use neon::types::{
	JsArray,
	JsFunction,
	JsNumber,
	JsObject,
	JsString,
	JsUndefined,
	JsValue,
};

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

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
	let arg0 = cx.argument(0)?;

	let req: Request = neon_serde::from_value(&mut cx, arg0)?;
	let req_body: HelloRequest = serde_json::from_slice(&req.body)
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	let res = HelloResponse {
		greeting: format!("Hello, {}!", req_body.name),
	};

	let res_body = serde_json::to_vec(&res)
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	let ret = neon_serde::to_value(&mut cx, &ByteBuf::from(res_body))?;

	Ok(ret)
}

fn parse(mut cx: FunctionContext) -> JsResult<JsValue> {
	let s = cx.argument::<JsString>(0)?;

	let o: serde_json::Value = serde_json::from_str(&s.value())
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	let o = neon_serde::to_value(&mut cx, &o)?;

	Ok(o)
}

fn stringify(mut cx: FunctionContext) -> JsResult<JsString> {
	let o = cx.argument(0)?;

	let o: serde_json::Value = neon_serde::from_value(&mut cx, o)?;
	let s = serde_json::to_string(&o)
		.or_else(|err| cx.throw_error(&err.to_string()))?;

	Ok(JsString::new(&mut cx, &s))
}

struct SuccessTask;

impl Task for SuccessTask {
	type Output = i32;
	type Error = String;
	type JsEvent = JsNumber;

	fn perform(&self) -> Result<Self::Output, Self::Error> {
		Ok(17)
	}

	fn complete(
		self,
		mut cx: TaskContext,
		result: Result<Self::Output, Self::Error>,
	) -> JsResult<Self::JsEvent> {
		Ok(JsNumber::new(&mut cx, result.unwrap() as f64))
	}
}

fn perform_async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let f = cx.argument::<JsFunction>(0)?;
	SuccessTask.schedule(f);
	Ok(JsUndefined::new())
}

fn array_process(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let arr = cx.argument::<JsArray>(0)?;

	for i in 0..arr.len() {
		let item = arr
			.get(&mut cx, i)?
			.downcast::<JsObject>()
			.or_throw(&mut cx)?;
		let operator = item
			.get(&mut cx, "operator")?
			.downcast::<JsString>()
			.or_throw(&mut cx)?
			.value();
		let value = item
			.get(&mut cx, "value")?
			.downcast::<JsString>()
			.or_throw(&mut cx)?
			.value();

		match operator.as_str() {
			"print" => {
				println!("{}", value);
			}
			_ => {
				let msg = format!("Unsupported operator: {}", operator);

				return cx.throw_error(&msg);
			}
		}
	}

	Ok(JsUndefined::new())
}

#[derive(Deserialize)]
#[serde(tag = "operator")]
enum Operation {
	#[serde(rename = "print")]
	Print { value: String },
}

fn array_process_serde(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let arg0 = cx.argument(0)?;
	let ops: Vec<Operation> = neon_serde::from_value(&mut cx, arg0)?;

	for op in ops {
		match op {
			Operation::Print { value } => {
				println!("{}", value);
			}
		}
	}

	Ok(JsUndefined::new())
}

register_module!(mut cx, {
	cx.export_function("parse", parse)?;
	cx.export_function("stringify", stringify)?;
	cx.export_function("hello", hello)?;
	cx.export_function("performAsyncTask", perform_async_task)?;
	cx.export_function("arrayProcess", array_process)?;
	cx.export_function("arrayProcessSerde", array_process_serde)?;

	Ok(())
});
