use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

mod changed_files_row;

pub(crate) struct Model {
	changed_files: FactoryVecDeque<changed_files_row::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	AddChangedFileRow(String, git::Status),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		adw::Bin {
			if model.changed_files.is_empty() {
				adw::StatusPage {
					set_icon_name: Some("check-round-outline"),  // FIXME: "Unkwown icon"
					set_title: "No changes",
					set_description: Some("Working tree clean."),
				}
			} else {
				gtk::ScrolledWindow {
					gtk::Box {
						// This `Box` prevents the list background from being the size of the whole view.
						// It's only noticeable in small enough lists.
						set_orientation: gtk::Orientation::Vertical,

						set_margin_all: 12,

						#[local_ref]
						changed_files_list_box -> gtk::ListBox {
							add_css_class: "boxed-list",
						},
					},
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let changed_files = FactoryVecDeque::builder()
			.launch_default()
			.detach();
		let model = Self { changed_files };

		let changed_files_list_box = model.changed_files.widget();
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Input::AddChangedFileRow(file_path, file_status) => {
				let row = changed_files_row::Init {
					file_path,
					file_status,
				};
				self.changed_files.guard().push_back(row);
			}
		}
	}
}
