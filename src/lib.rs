use neon::prelude::*;

fn string(mut cx: FunctionContext) -> JsResult<JsString> {
    let string = cx.argument::<JsString>(0)?;
    Ok(string)
}

fn number(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let number = cx.argument::<JsNumber>(0)?;
    Ok(number)
}

fn boolean(mut cx: FunctionContext) -> JsResult<JsValue> {
    if let Ok(boolean) = cx.argument::<JsBoolean>(0) {
        let js_value_valid: JsValue = boolean.upcast();

        Ok(js_value_valid)
    } else {
        let invalid_message: Handle<JsString> = cx.string("Incorrect type");
        let js_value_invalid: JsValue = invalid_message.upcast();

        Ok(js_value_invalid)
    }
}

fn null(mut cx: FunctionContext) -> JsResult<JsNull> {
    let null = cx.argument::<JsNull>(0)?;
    Ok(null)
}

fn undefined(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let undefined = cx.argument::<JsUndefined>(0)?;
    Ok(undefined)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("string", string)?;
    cx.export_function("number", number)?;
    cx.export_function("boolean", boolean)?;
    cx.export_function("null", null)?;
    cx.export_function("undefined", undefined)?;
    Ok(())
}
