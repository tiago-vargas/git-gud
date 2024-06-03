use gtk::{gio, prelude::*};
use relm4::prelude::*;

pub(crate) struct Model {
	selected_repository: Option<gio::File>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ShowOpenRepoDialog,
	PlaceholderAction(gio::File),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
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

					connect_clicked[sender] => move |_| {
						sender.input(Self::Input::ShowOpenRepoDialog)
					},
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			selected_repository: None,
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::ShowOpenRepoDialog => {
				let app = relm4::main_application();
				let main_window = app
					.windows()
					.first()
					.expect(
						"Event should have been triggered by last focused window, thus first item",
					)
					.clone();

				let home = std::env::var("HOME").expect("System should have set `HOME` on login");
				let dialog = gtk::FileDialog::builder()
					.title("Open Repository")
					.initial_folder(&gio::File::for_path(home))
					.modal(true)
					.build();
				dialog.select_folder(
					Some(&main_window),
					None::<&gio::Cancellable>,
					move |result| {
						if let Ok(selected_folder) = result {
							sender.input(Self::Input::PlaceholderAction(selected_folder));
						}
					},
				)
			}
			Self::Input::PlaceholderAction(repository) => {
				self.selected_repository = Some(repository);
			}
		}
	}
}
