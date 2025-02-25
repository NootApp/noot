use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Event<'a> {
    name: &'a str,
    context: Option<&'a str>,
    ok: bool
}


pub fn announce_event<'a>(app: AppHandle, name: &'a str, context: Option<&'a str>, success: bool) {
    


    app.emit_to("main", "event-stream", Event{
        name,
        context,
        ok: success
    });
}
