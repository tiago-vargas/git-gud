use relm4::{factory::FactoryVecDeque, prelude::*};

mod changed_files_row;

pub(crate) struct Model {
	changed_files: FactoryVecDeque<changed_files_row::Model>,
}

pub(crate) struct Init;

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();

	view! {
		adw::StatusPage {
			set_title: "Status",
			set_description: Some("Status of the repository."),
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

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}
}
