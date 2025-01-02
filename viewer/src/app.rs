use crate::prelude::*;
use crate::prelude::subclass::*;
use crate::appwindow::AppWindow;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
      @extends gio::Application, gtk4::Application;
}

mod imp {
    use super::*;
    use crate::widgets::fileinfo::FileInfoWidget;

    pub struct App;

    impl Default for App {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "App";
        type Type = super::App;
        type ParentType = gtk4::Application;
    }

    impl ObjectImpl for App {}

    impl ApplicationImpl for App {
        fn startup(&self) {
            self.parent_startup();
            self.setup_buildables();
        }

        fn activate(&self) {
            self.parent_activate();
            self.new_app_window();
        }
    }

    impl GtkApplicationImpl for App {}

    impl App {
        fn setup_buildables(&self) {
            AppWindow::static_type();
            FileInfoWidget::static_type();
        }

        pub fn new_app_window(&self) {
            let app_window = AppWindow::new(self.obj().upcast_ref());
            app_window.init();
            app_window.present();
        }
    }
}

impl App {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "io.github.arteme.l6t-rs.viewer")
            .property("resource-base-path", "/io/github/arteme/l6t-rs/viewer/")
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            //.property("register-session", true)
            .build()
    }
}
