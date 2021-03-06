use gtk::{
    gio::{ActionGroup, ActionMap},
    glib::{self, Object},
    ApplicationWindow, Root, Widget,
};

use crate::Application;

mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
        CompositeTemplate,
    };
    use he::{prelude::*, subclass::prelude::*, ApplicationWindow};
    use log::debug;

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Solar/window.ui")]
    pub struct Window {}

    impl Default for Window {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "SolarWindow";
        type Type = super::Window;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl HeApplicationWindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl WindowImpl for Window {}
    impl ObjectImpl for Window {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("HeWindow<Window>::realize");
            });
        }
    }
    impl WidgetImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends Widget, gtk::Window, ApplicationWindow, he::ApplicationWindow, ActionMap, ActionGroup,
        @implements Root;

}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(&Application::default())
    }
}
