use crate::prelude::*;
use crate::prelude::subclass::*;
use crate::model::DataGroup;

glib::wrapper! {
    pub struct Page(ObjectSubclass<imp::Page>)
      @extends gtk4::Widget, gtk4::Box
    ;
}

mod imp {
    use crate::group::Group;
    use super::*;

    pub struct Page {
        size_group: gtk4::SizeGroup,
    }

    impl Default for Page {
        fn default() -> Self {
            let size_group = gtk4::SizeGroup::new(gtk4::SizeGroupMode::Both);
            Page { size_group }
        }
    }

    impl Page {
        fn init(&self) {
        }

        pub(super) fn set_groups(&self, groups: Vec<DataGroup>) {
            let obj = self.obj();
            let box_ = obj.upcast_ref::<gtk4::Box>();
            while let Some(child) = box_.first_child() {
                box_.remove(&child);
            }
            for g in groups {
                let group = Group::new(&self.size_group);
                group.set_label(&g.title);
                group.set_items(g.values);
                box_.append(&group);
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Page {
        const NAME: &'static str = "DataPage";
        type Type = super::Page;
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for Page {
        fn constructed(&self) {
            self.parent_constructed();
            self.init();
        }
    }
    impl WidgetImpl for Page {}
    impl BoxImpl for Page {}
}

impl Page {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("orientation", gtk4::Orientation::Vertical)
            .build()
    }

    pub fn set_groups(&self, groups: Vec<DataGroup>) {
        self.imp().set_groups(groups);
    }
}