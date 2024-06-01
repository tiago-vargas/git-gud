use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model;

pub(crate) struct Init;

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();

	view! {
		adw::StatusPage {
			set_icon_name: Some("folder-symbolic"),
			set_title: "No Repository Selected",

			gtk::CenterBox {
				// `StatusPage` takes 1 child widget, which expands to its width.
				// Having just the button as the child, makes it stretched just too much.
				// Wraping in a `CenterBox` is a workaround to make the button small.
				#[wrap(Some)]
				set_center_widget = &gtk::Button {
					set_label: "Select Repository…",
					add_css_class: "suggested-action",
					add_css_class: "pill",
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self;

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}
}
