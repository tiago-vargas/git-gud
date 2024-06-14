use adw::prelude::*;
use relm4::{factory::FactoryView, prelude::*};

pub(crate) struct Model {
	file_path: String,
}

pub(crate) struct Init {
	pub(crate) file_path: String,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();

	type CommandOutput = ();
	type ParentWidget = gtk::ListBox;

	view! {
		adw::ActionRow {
			set_title: file_name(&self.file_path),
			set_subtitle: &self.file_path,
		}
	}

	fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
		Self {
			file_path: init.file_path,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &Self::Index,
		root: Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		_sender: FactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
		let () = message;
	}
}

fn file_name(path: &str) -> &str {
	std::path::Path::new(path)
		.file_name()
		.expect("File should have a name")
		.to_str()
		.expect("Name should be valid UTF-8")
}
