use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

mod changed_files_row;

pub(crate) struct Model {
	changed_files: FactoryVecDeque<changed_files_row::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	AddChangedFileRow(String),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		adw::StatusPage {
			set_title: "Status",
			set_description: Some("Status of the repository."),

			#[local_ref]
			changed_files_list_box -> gtk::ListBox {
				add_css_class: "boxed-list",
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let changed_files = FactoryVecDeque::builder()
			.launch_default()
			.detach();
		let model = Self { changed_files };

		let changed_files_list_box = model.changed_files.widget();
		let widgets = view_output!();

		sender.input(Self::Input::AddChangedFileRow(String::from("File 1.rs")));
		sender.input(Self::Input::AddChangedFileRow(String::from("File 2.rs")));
		sender.input(Self::Input::AddChangedFileRow(String::from("File 3.rs")));

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Input::AddChangedFileRow(file_name) => {
				self.changed_files
					.guard()
					.push_back(changed_files_row::Init { name: file_name });
			}
		}
	}
}
