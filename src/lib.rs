#![warn(bare_trait_objects)]
#[macro_use]
extern crate guid;

mod lng;
mod clip;

use log::*;
use farmanager::FarPlugin;
use farmanager::*;
use crate::lng::Lng;
use std::io;
use std::ptr;
use std::str;
use clipboard_win::Clipboard;
use farmanager::panel::control::{get_panel_item, set_selection};
use farmanager::panel::{Panel, PanelInfo};
use panel::control::{begin_selection, end_selection, get_panel_info};
use simplelog::WriteLogger;
use simplelog::LevelFilter;
use simplelog::Config;
use widestring::WideCString;
use clip::get_filenames_from_selection;

plugin!(Plugin);

struct Plugin {
    guid: GUID
}

impl Plugin {
    fn new() -> Plugin {
        init_logger();
        Plugin {
            guid: guid_from_tuple(guid_parts! {"1CDF244B-5616-436A-9506-A31315BA839B"})
        }
    }

    fn display_error(error: &str) {
        basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN, None,
                       basic::MessageItems::Lines(vec!(
                           basic::get_msg(&Lng::ErrorTitle),
                           WideString::from(error),
                           basic::get_msg(&Lng::MessageButton)
                       )), 1);
    }

    fn needs_selection(selected_file_names: &Vec<String>, item_file_name: WideString) -> bool {
        let string = item_file_name.to_string();
        selected_file_names
            .into_iter()
            .find(|i| (*i).eq(&string))
            .is_some()
    }

    fn select_filenames_on_panel(filenames: &Vec<String>, panel_info: PanelInfo) {
        let item_count = panel_info.items_number;
        let mut selected_count = 0;
        begin_selection(Panel::Active);
        for i in 1..item_count {
            match get_panel_item(Panel::Active, i) {
                Ok(item) => {
                    if Self::needs_selection(filenames, item.file_name) {
                        set_selection(Panel::Active, i, true);
                        selected_count += 1;
                    }
                },
                Err(e) => Self::display_error(&e.to_string())
            }
        }
        end_selection(Panel::Active);
        basic::message(basic::FARMESSAGEFLAGS::FMSG_LEFTALIGN, None,
                       basic::MessageItems::Lines(vec!(
                           basic::get_msg(&Lng::MessageTitle),
                           basic::get_msg(&Lng::MessageLine0),
                           basic::get_msg(&Lng::MessageLine1),
                           WideString::from(selected_count.to_string()),
                           WideString::from(basic::DIALOG_SEPARATOR),
                           basic::get_msg(&Lng::MessageButton)
                       )), 1);
    }
}

impl FarPlugin for Plugin {

    fn basic_exports(&mut self) -> &mut dyn basic::ExportFunctions {
        self
    }
}

impl basic::ExportFunctions for Plugin {

    fn get_global_info(&mut self) -> basic::GlobalInfo {
        basic::GlobalInfo {
            min_far_version: basic::VersionInfo {
                    major: FARMANAGERVERSION_MAJOR,
                    minor: FARMANAGERVERSION_MINOR,
                    revision: FARMANAGERVERSION_REVISION,
                    build: FARMANAGERVERSION_BUILD,
                    stage: FARMANAGERVERSION_STAGE
                },
            version: basic::VersionInfo {
                    major: 0,
                    minor: 0,
                    revision: 1,
                    build: 1,
                    stage: basic::VersionStage::VS_ALPHA
                },
            guid: self.guid,
            title: WideString::from("Select from Clipboard"),
            description: WideString::from("Select from Clipboard plugin, written in Rust"),
            author: WideString::from("62mkv <62mkv@mail.ru>"),
        }
    }

    fn get_plugin_info(&mut self) -> basic::PluginInfo {
        basic::PluginInfo {
            flags: basic::PLUGIN_FLAGS::PF_NONE,
            command_prefix: None,
            plugin_menu: vec!(basic::MenuItem {
                guid: guid_from_tuple(guid_parts!{"5A4ED208-51BD-4165-8660-9AC0455EE546"}),
                label: basic::get_msg(&Lng::MenuItemTitle)
            }),
            disk_menu: Vec::new(),
            plugin_config: Vec::new()
        }
    }

    fn open(&mut self, open_from: basic::OpenFrom) -> HANDLE {
        trace!(">open_from_plugins_menu()");
        match open_from {
            basic::OpenFrom::PluginsMenu => {
                match Clipboard::new_attempts(3) {
                    Ok(_) => {
                        match get_filenames_from_selection() {
                            Ok(ref filenames) => {
                                match get_panel_info(Panel::Active) {
                                    Ok(panel_info) => Self::select_filenames_on_panel(filenames, panel_info),
                                    Err(e) => Self::display_error(&e.to_string())
                                }
                            },
                            Err(e) => Self::display_error(&e.to_string())
                        }
                    },
                    Err(e) => Self::display_error(&e.to_string())
                }
            },
            _ => {}
        };
        trace!("<open_from_plugins_menu()");
        return ptr::null_mut();
    }


}

fn init_logger() {
    WriteLogger::init(LevelFilter::Trace, Config::default(), io::LineWriter::new(Logger)).unwrap();
}

struct Logger;

impl io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let message = str::from_utf8(&buf).unwrap();
            kernel32::OutputDebugStringW(WideCString::from_str(message).unwrap().as_ptr());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn guid_from_tuple(guid_tuple: (u32, u16, u16, [u8; 8])) -> GUID {
    GUID {
        Data1: guid_tuple.0,
        Data2: guid_tuple.1,
        Data3: guid_tuple.2,
        Data4: guid_tuple.3
    }
}
