use chrono::prelude::*;
use chrono_tz::Europe::Zurich;
use schemars::{schema_for, JsonSchema};
use serde::Deserialize;

wit_bindgen::generate!({
    path: "../wg_display_widget_wit/wit",
    world: "widget"
});

#[derive(JsonSchema, Deserialize)]
struct WidgetConfig {}

const WIDGET_NAME: &str = "Time";

struct MyWidget;

impl Widget for MyWidget {
    fn get_name() -> wit_bindgen::rt::string::String {
        WIDGET_NAME.into()
    }

    fn run(_: WidgetContext) -> WidgetResult {
        let now = clocks::now();
        let naive = NaiveDateTime::from_timestamp_opt(now.seconds as i64, now.nanoseconds);
        if naive.is_none() {
            return WidgetResult {
                data: "Invalid timestamp".into(),
            };
        }

        let datetime = Zurich.from_utc_datetime(&naive.unwrap());
        let newdate = datetime.format("%H:%M:%S %d.%m.%Y");
        WidgetResult {
            data: newdate.to_string(),
        }
    }

    fn get_config_schema() -> wit_bindgen::rt::string::String {
        let schema = schema_for!(WidgetConfig);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    fn get_version() -> wit_bindgen::rt::string::String {
        "1.0.2".into()
    }

    fn get_run_update_cycle_seconds() -> u32 {
        1
    }
}

export_widget!(MyWidget);
