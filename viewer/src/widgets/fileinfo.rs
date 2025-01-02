use crate::prelude::*;
use crate::prelude::subclass::*;

glib::wrapper! {
    pub struct FileInfoWidget(ObjectSubclass<imp::FileInfoWidget>)
      @extends gtk4::Widget, gtk4::Grid;
}

mod imp {
    use super::*;
    use gtk4::CompositeTemplate;

    #[derive(CompositeTemplate)]
    #[template(resource = "/io/github/arteme/l6t-rs/viewer/ui/fileinfo.ui")]
    pub struct FileInfoWidget {}

    impl Default for FileInfoWidget {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileInfoWidget {
        const NAME: &'static str = "FileInfoWidget";
        type Type = super::FileInfoWidget;
        type ParentType = gtk4::Grid;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FileInfoWidget {}
    impl WidgetImpl for FileInfoWidget {}
    impl GridImpl for FileInfoWidget {}
}