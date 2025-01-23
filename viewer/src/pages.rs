use l6t::symbolic::value::Value;

use crate::file::Patch;
use crate::page::Page;
use crate::prelude::*;
use crate::{model, resource};

pub fn empty_page() -> gtk4::Widget {
    gtk4::Builder::from_resource(&resource("ui/empty-page.ui"))
        .object::<gtk4::Widget>("empty_page_widget")
        .unwrap()
}

pub fn patch_page(patch: &Patch) -> gtk4::Widget {
    let mut device_groups = vec![];
    let device_group = {
        let p = &patch.patch;
        let m = &p.meta;

        model::shorthand::group("Device")
            .item("Device", format!("{:#04x}", p.target_device.midi_id))
            .item("Name", &p.target_device.name)
            .item_ne("Author", &m.author)
            .item_ne("Guitarist", &m.guitarist)
            .item_ne("Band", &m.band)
            .item_ne("Song", &m.song)
            .item_ne("Style", &m.style)
            .item_ne2("Pickup", &m.pickup_position, " - ", &m.pickup_style)
            .item_ne("Amp", &m.amp_name)
            .item_ne2("App", &m.creator_app, " ", &m.creator_app_version)
            .item_ne("Comments", &m.comments)
            .into()
    };
    device_groups.push(device_group);

    for g in &patch.values {
        let mut group = model::shorthand::group(&g.name);

        for (n, v) in &g.values {
            let extra = match v.get_simple() {
                Value::Int(v) => format!(" ({:#04x})", v),
                _ => String::new()
            };
            let tooltip = format!("{}: {}{}", v.get_simple_type(), v.get_simple(), extra);
            group = group.item(n, &v.to_string()).tooltip(&tooltip);
        }
        device_groups.push(group.into());
    }

    let page = Page::new();
    page.set_groups(device_groups);

    return page.upcast();
}