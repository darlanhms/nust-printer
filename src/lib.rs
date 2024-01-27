use neon::{prelude::*, types::buffer::TypedArray};

#[cfg(target_family = "windows")]
mod windows;

mod printer;

fn get_printers(mut cx: FunctionContext) -> JsResult<JsArray> {
    let printers_list = windows::get_printers();
    let js_printers = JsArray::new(&mut cx, printers_list.len() as u32);

    for (i, printer) in printers_list.iter().enumerate() {
        let printer_obj = cx.empty_object();
        let name = cx.string(&printer.name);

        printer_obj.set(&mut cx, "name", name)?;

        js_printers.set(&mut cx, i as u32, printer_obj)?;
    }

    Ok(js_printers)
}

fn print_direct(mut cx: FunctionContext) -> JsResult<JsNull> {
    let printer_config = cx.argument::<JsObject>(0)?;

    let printer_name_raw: Handle<JsString> = printer_config.get(&mut cx, "printer")?;
    let data_raw: Handle<JsBuffer> = printer_config.get(&mut cx, "data")?;

    let printer_name = printer_name_raw.value(&mut cx);
    let data = data_raw.as_slice(&mut cx);

    windows::print_direct(printer_name, data);


    Ok(JsNull::new(&mut cx))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getPrinters", get_printers)?;
    cx.export_function("printDirect", print_direct)?;

    Ok(())
}
