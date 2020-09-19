extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    // Attempt to cast the first argument to a JsString. Then
    // get the value if cast is successul
    let name = cx.argument::<JsString>(0)?.value();
    let return_val = format!("Hello {}!", name);
    Ok(cx.string(return_val))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
