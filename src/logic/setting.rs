use crate::{
    config::{self, Config},
    db,
    logic::message::{async_message_success, async_message_warn},
    message_warn,
    slint_generatedAppWindow::{AppWindow, Logic, Store, Theme},
    util::{http, translator::tr},
    version,
};
use anyhow::{anyhow, bail, Result};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Weak};
use std::time::Duration;
use uuid::Uuid;

pub fn init(ui: &AppWindow) {
    init_setting(&ui);

    ui.global::<Store>()
        .set_is_first_run(config::is_first_run());

    ui.global::<Store>()
        .set_is_show_landing_page(config::is_first_run());

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_get_setting_ui(move || {
        let ui = ui_handle.unwrap();
        ui.global::<Store>().get_setting_ui()
    });

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_set_setting_ui(move |mut setting| {
        let font_size = u32::min(50, u32::max(10, setting.font_size.parse().unwrap_or(16)));
        setting.font_size = slint::format!("{}", font_size);

        ui_handle
            .unwrap()
            .global::<Store>()
            .set_setting_ui(setting.clone());

        let mut all = config::all();
        all.ui.font_size = font_size.into();
        all.ui.font_family = setting.font_family.into();
        all.ui.language = setting.language.into();
        all.ui.is_dark = setting.is_dark;
        _ = config::save(all);
    });

    ui.global::<Logic>()
        .on_tr(move |_is_cn, text| tr(text.as_str()).into());
}

fn init_setting(ui: &AppWindow) {
    let config = config::ui();
    let mut ui_setting = ui.global::<Store>().get_setting_ui();

    let font_size = u32::min(50, u32::max(10, config.font_size));
    ui_setting.font_size = slint::format!("{}", font_size);
    ui_setting.font_family = config.font_family.into();
    ui_setting.language = config.language.into();
    ui_setting.is_dark = config.is_dark;

    ui.global::<Theme>().invoke_set_dark(config.is_dark);
    ui.global::<Store>().set_setting_ui(ui_setting);
}
