use crate::prelude::*;
use crate::prelude::subclass::*;
use crate::model::DataItem;

glib::wrapper! {
    pub struct Group(ObjectSubclass<imp::Group>)
      @extends gtk4::Widget, gtk4::Frame
    ;
}

mod imp {
    use std::cell::{OnceCell, RefCell};
    use std::fmt::Debug;
    use super::*;

    #[derive(Default)]
    pub struct Group {
        expander: OnceCell<gtk4::Expander>,
        size_group: OnceCell<gtk4::SizeGroup>,
    }

    impl Group {
        fn init(&self) {
            let obj = self.obj();
            let frame = obj.upcast_ref::<gtk4::Frame>();
            let expander = gtk4::Expander::builder()
                .expanded(true)
                .build();
            frame.set_child(Some(&expander));

            self.expander.set(expander).unwrap();
        }

        pub fn set_items(&self, items: Vec<DataItem>) {
            let grid = gtk4::Grid::new();
            let sg = self.size_group.get().unwrap();

            for (n, item) in items.into_iter().enumerate() {
                let builder = || gtk4::Label::builder()
                    .hexpand(true)
                    .hexpand_set(true)
                    .xalign(0.0);
                let label = builder().label(&item.label).build();
                let value = builder().label(&item.value).build();
                if let Some(tooltip) = item.tooltip {
                    value.set_tooltip_text(Some(&tooltip));
                }
                grid.attach(&label, 0, n as i32, 1, 1);
                grid.attach(&value, 1, n as i32, 1, 1);

                sg.add_widget(&label);
                sg.add_widget(&value);
            }

            self.expander.get().unwrap().set_child(Some(&grid));
        }

        pub fn set_label(&self, label: &str) {
            self.expander.get().unwrap().set_label(Some(label));
        }
        pub fn set_size_group(&self, size_group: &gtk4::SizeGroup) {
            self.size_group.set(size_group.clone()).unwrap();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Group {
        const NAME: &'static str = "Group";
        type Type = super::Group;
        type ParentType = gtk4::Frame;
    }

    impl ObjectImpl for Group {
        fn constructed(&self) {
            self.parent_constructed();
            self.init();
        }
    }
    impl WidgetImpl for Group {}
    impl FrameImpl for Group {}
}

impl Group {
    pub fn new(size_group: &gtk4::SizeGroup) -> Self {
        let g: Self = glib::Object::builder()
            .build();
        g.imp().set_size_group(size_group);
        g
    }

    pub fn set_label(&self, label: &str) {
        self.imp().set_label(label);
    }

    pub fn set_items(&self, items: Vec<DataItem>) {
        self.imp().set_items(items);
    }
}