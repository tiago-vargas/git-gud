use gtk::{gio, prelude::*};
use relm4::prelude::*;

pub(crate) struct Model {
	selected_repository: Option<gio::File>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ShowOpenRepoDialog,
	PlaceholderAction(Option<gio::File>),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		adw::Bin {
			if model.selected_repository.is_none() {
				adw::StatusPage {
					set_icon_name: Some("folder-symbolic"),
					set_title: "No Repository Selected",
					set_description: Some("Use the button below to open a repository."),

					gtk::CenterBox {
						// `StatusPage` takes 1 child widget, which expands to its width.
						// Having just the button as the child, makes it stretched just too much.
						// Wraping in a `CenterBox` is a workaround to make the button small.
						#[wrap(Some)]
						set_center_widget = &gtk::Button {
							set_label: "Open Repository",
							add_css_class: "suggested-action",
							add_css_class: "pill",

							connect_clicked[sender] => move |_| {
								sender.input(Self::Input::ShowOpenRepoDialog)
							},
						},
					},
				}
			} else {
				adw::StatusPage {
					#[watch] set_title: model.selected_repository.clone().unwrap().path().unwrap().to_str().unwrap(),
				}
			}
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

				// Use `gtk::FileDialog` instead when Relm4 ship with a new GTK version.
				// `gtk::FileChooserDialog` is deprecated from 4.10 onwards, but we ain't there yet.
				let dialog = gtk::FileChooserDialog::builder()
					.title("Open Repository")
					.transient_for(&main_window)
					.action(gtk::FileChooserAction::SelectFolder)
					.modal(true)
					.build();
				dialog.add_button("Open", gtk::ResponseType::Accept);
				dialog.add_button("Cancel", gtk::ResponseType::Cancel);
				dialog.connect_response(move |this, response| match response {
					gtk::ResponseType::Accept => {
						let selected_folder = this.file();
						sender.input(Self::Input::PlaceholderAction(selected_folder));
						this.close();
					}
					gtk::ResponseType::Cancel => this.close(),
					gtk::ResponseType::DeleteEvent => (),
					other => unimplemented!("Unexpected response type: {other:?}"),
				});

				dialog.present();
			}
			Self::Input::PlaceholderAction(repository) => {
				self.selected_repository = repository;
			}
		}
	}
}
