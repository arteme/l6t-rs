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
    use std::cell::{OnceCell, RefCell};
    use gio::ActionEntry;
    use gtk4::{CompositeTemplate, FileFilter, gdk};
    use gtk4::prelude::TreeViewExt;
    use crate::pages::{empty_page, patch_page};
    use crate::file::{File, Selection};
    use crate::loading::load_file;
    use crate::prelude::gio::InputStream;
    use crate::resource;
    use crate::util::{ref_remap, uri_after_scheme};

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/io/github/arteme/l6t-rs/viewer/ui/appwindow.ui")]
    pub struct AppWindow {
        file_contents: RefCell<Option<File>>,
        subtitle_label: OnceCell<gtk4::Label>,

        #[template_child]
        header_bar: TemplateChild<gtk4::HeaderBar>,
        #[template_child]
        open_file_button: TemplateChild<gtk4::Button>,
        #[template_child]
        tree_view: TemplateChild<gtk4::TreeView>,
        #[template_child]
        webview_parent: TemplateChild<gtk4::ScrolledWindow>,
    }

    impl AppWindow {
        fn init(&self) {
            self.init_actions();
            self.init_tree_view();
            //self.init_webview();

            let drop_target = gtk4::DropTarget::builder()
                .actions(gdk::DragAction::COPY)
                .formats(&gdk::ContentFormats::for_type(gio::File::static_type()))
                .build();

            drop_target.connect_drop(glib::clone!(
                #[weak(rename_to = w)] self,
                #[upgrade_or] false,
                move |_, drop, _, _ | {
                    let mut accept = false;
                    match drop.get::<gio::File>() {
                        Ok(file) => {
                            w.load(file);
                            accept = true;
                        }
                        Err(e) => {
                            error!("Failed to get drop: {}", e);
                        }
                    }
                    accept
                }
            ));

            let obj = self.obj();
            obj.add_controller(drop_target);

            self.webview_parent.get().set_child(Some(&empty_page()));
        }

        fn init_actions(&self) {
            let open_action = ActionEntry::builder("open")
                .activate(clone!(
                #[weak(rename_to = w)] self,
                move |_, _, _| {
                    glib::spawn_future_local(async move {
                        match w.open_file_dialog().await {
                                Ok(file) => {
                                    w.load(file);
                                }
                                Err(e) => {
                                    error!("File loading failed: {e}");
                                }
                        }
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
                move |_tree_view, path, _column| {
                    let path = path.indices();
                    w.select(&path);
                }
            ));
        }

        fn set_subtitle(&self, subtitle: &str) {
            let subtitle_label = self.subtitle_label.get_or_init(|| {
                let title_box = gtk4::Box::builder()
                    .orientation(gtk4::Orientation::Vertical)
                    .build();
                let title_label = gtk4::Label::builder()
                    .css_classes(["title"])
                    .build();
                let subtitle_label = gtk4::Label::builder()
                    .css_classes(["subtitle"])
                    .build();
                title_box.append(&title_label);
                title_box.append(&subtitle_label);
                self.header_bar.set_title_widget(Some(&title_box));

                let obj = self.obj();
                let w = obj.upcast_ref::<gtk4::ApplicationWindow>();
                if let Some(title) = w.title() {
                    title_label.set_label(&title);
                }

                subtitle_label
            });

            subtitle_label.set_label(subtitle);
        }

        fn select(&self, path: &[i32]) {
            let w = match self.get_patch(path) {
                Selection::Patch(p) => patch_page(&p),
                _ => gtk4::Box::builder().build().upcast(),
            };
            self.webview_parent.get().set_child(Some(&w));
        }

        fn get_patch(&self, path: &[i32]) -> Selection {
            let file_contents = self.file_contents.borrow();

            match file_contents.as_ref() {
                None => {
                    return Selection::None;
                }
                Some(File::Patch(patch)) => {
                    let patch = ref_remap(&file_contents, patch);
                    return Selection::Patch(patch);
                }
                Some(File::Bundle(b)) => {
                    let Some(bank) = b.banks.get(path[0] as usize) else {
                        return Selection::None;
                    };
                    if path.len() == 1 {
                        let bank = ref_remap(&file_contents, bank);
                        return Selection::Bank(bank);
                    }
                    let Some(patch) = bank.patches.get(path[1] as usize) else {
                        return Selection::None;
                    };
                    let patch = ref_remap(&file_contents, patch);
                    return Selection::Patch(patch);
                }
            }
        }

        fn load(&self, file: gio::File) {
            let path = file.path()
                .map(|p| p.to_str().unwrap().to_string())
                .unwrap_or_default();
            match load_file(file) {
                Ok(contents) => {
                    self.loaded(contents, path);
                }
                Err(e) => {
                    error!("Failed to load {}: {}", path, e);
                }
            }
        }

        fn loaded(&self, file: File, path: String) {
            self.file_contents.replace(Some(file));
            let m = self.tree_view.model().unwrap().dynamic_cast::<gtk4::TreeStore>().unwrap();
            m.clear();

            let file = self.file_contents.borrow();
            self.set_subtitle(&path);

            match file.as_ref().unwrap() {
                File::Patch(p) => {
                    let name = &p.patch.target_device.name;
                    m.insert_with_values(
                        None, None,
                        &[(0, &0), (1, &name)]
                    );
                    // When it is only one patch, select it right away
                    let path = gtk4::TreePath::from_string("0").unwrap();
                    self.tree_view.selection().select_path(&path);
                    self.select(&[0]);
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

                    self.select(&[]);
                }
            }
        }

        async fn open_file_dialog(&self) -> Result<gio::File> {
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

            dialog.open_future(Some(self.obj().upcast_ref::<gtk4::Window>())).await
                .context("File open failed")
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
}