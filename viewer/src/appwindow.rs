#![allow(deprecated)]

use crate::prelude::*;
use crate::prelude::subclass::*;

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
      @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
      @implements gio::ActionGroup, gio::ActionMap
    ;
}

mod imp {
    use super::*;
    use std::cell::RefCell;
    use gio::ActionEntry;
    use gtk4::{CompositeTemplate, FileFilter};
    use gtk4::prelude::TreeViewExt;
    use crate::file::{File, Selection};
    use crate::loading::load_file;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/io/github/arteme/l6t-rs/viewer/ui/appwindow.ui")]
    pub struct AppWindow {
        file_contents: RefCell<Option<File>>,

        #[template_child]
        open_file_button: TemplateChild<gtk4::Button>,
        #[template_child]
        tree_view: TemplateChild<gtk4::TreeView>,
    }

    impl AppWindow {
        fn init(&self) {
            self.init_actions();
            self.init_tree_view();
        }

        fn init_actions(&self) {
            let open_action = ActionEntry::builder("open")
                .activate(clone!(
                #[weak(rename_to=w)]
                self,
                move |_, _, _| {
                    glib::spawn_future_local(async move {
                            let f = w.open_file_dialog().await;
                            w.loaded(f.expect("something"));
                    });
                }
            ))
                .build();

            self.obj().add_action_entries([open_action]);
        }

        fn init_tree_view(&self) {
            let Some(tree_view) = self.tree_view.try_get() else { return };

            let model = gtk4::TreeStore::new(&[u32::static_type(), String::static_type()]);
            tree_view.set_model(Some(&model));
            tree_view.set_headers_visible(false);

            let column = gtk4::TreeViewColumn::new();
            let renderer = gtk4::CellRendererText::new();
            column.pack_start(&renderer, true);
            column.add_attribute(&renderer, "text", 1);
            tree_view.append_column(&column);

            tree_view.connect_cursor_changed(|tree_view| {
                let (path, column) = TreeViewExt::cursor(tree_view);
                if path.is_none() || column.is_none() { return };
                tree_view.row_activated(&path.unwrap(), column.as_ref());
            });
            tree_view.connect_row_activated(glib::clone!(
                #[weak(rename_to=w)]
                self,
                move |tree_view, path, column| {
                    let path = path.indices();
                    w.select(&path);
                }
            ));
        }

        fn select(&self, path: &[i32]) {
            match self.file_contents.borrow().as_ref() {
                None => {}
                Some(File::Patch(p)) => {
                    self.selected(Selection::Patch(p));
                    return;
                }
                Some(File::Bundle(b)) => {
                    let Some(bank) = b.banks.get(path[0] as usize) else {
                        self.selected(Selection::None);
                        return;
                    };
                    if path.len() == 1 {
                        self.selected(Selection::Bank(bank));
                        return;
                    }
                    let Some(patch) = bank.patches.get(path[1] as usize) else {
                        self.selected(Selection::None);
                        return;
                    };
                    self.selected(Selection::Patch(patch));
                }
            }
        }

        fn selected(&self, sel: Selection) {
            println!("selected something!");
        }

        fn loaded(&self, file: File) {
            self.file_contents.replace(Some(file));
            let m = self.tree_view.model().unwrap().dynamic_cast::<gtk4::TreeStore>().unwrap();
            m.clear();

            let file = self.file_contents.borrow();
            println!("Loaded...");
            match file.as_ref().unwrap() {
                File::Patch(p) => {
                    let name = &p.patch.target_device.name;
                    m.insert_with_values(
                        None, None,
                        &[(0, &0), (1, &name)]
                    );
                }
                File::Bundle(b) => {
                    for bank in &b.banks {
                        let name = &bank.name;
                        let bank_iter = m.insert_with_values(
                            None, None,
                            &[(0, &0), (1, &name)]
                        );

                        for patch in &bank.patches {
                            let name = &patch.patch.target_device.name;
                            m.insert_with_values(
                                Some(&bank_iter), None,
                                &[(0, &0), (1, &name)]
                            );
                        }
                    }
                }
            }
        }

        async fn open_file_dialog(&self) -> Result<File> {
            let filter = gtk4::FileFilter::new();
            filter.set_name(Some("All supported formats (L6T, L6B, L6C)"));
            filter.add_pattern("*.l6t");
            filter.add_pattern("*.l6b");
            filter.add_pattern("*.l6c");

            let filter_list = gio::ListStore::new::<FileFilter>();
            filter_list.append(&filter);

            let dialog = gtk4::FileDialog::builder()
                .title("Open File")
                .modal(true)
                .accept_label("Open")
                .filters(&filter_list)
                .default_filter(&filter)
                .build();

            match dialog.open_future(Some(self.obj().upcast_ref::<gtk4::Window>())).await {
                Ok(f) => {
                    load_file(f)
                }
                Err(e) => {
                    bail!("Failed to load file: {e}");
                }
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
        type ParentType = gtk4::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AppWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.init();
        }
    }
    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {}
    impl ApplicationWindowImpl for AppWindow {}
}

impl AppWindow {
    pub fn new(app: &gtk4::Application) -> Self {
        glib::Object::builder()
            .property("application", app)
            .build()
    }

    pub fn init(&self) {
    }
}