use adw::prelude::*;
use relm4::{factory::FactoryView, prelude::*};

pub(crate) struct Model {
	file_path: String,
	file_status: git::Status,
}

pub(crate) struct Init {
	pub(crate) file_path: String,
	pub(crate) file_status: git::Status,
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

			set_title_lines: 1,
			set_subtitle_lines: 1,
			set_tooltip: &self.file_path,

			add_suffix = &gtk::Label {
				set_label: &label_from_status(&self.file_status),
				add_css_class: "monospace",
				set_tooltip: &format!("{:?}", &self.file_status),
			},
		}
	}

	fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
		Self {
			file_path: init.file_path,
			file_status: init.file_status,
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

fn label_from_status(status: &git::Status) -> String {
	let mut tuple = ("_", "_");

	if status.contains(git::Status::INDEX_MODIFIED) {
		tuple.0 = "M";
	} else if status.contains(git::Status::INDEX_TYPECHANGE) {
		tuple.0 = "T";
	} else if status.contains(git::Status::INDEX_NEW) {
		tuple.0 = "A";
	} else if status.contains(git::Status::INDEX_DELETED) {
		tuple.0 = "D";
	} else if status.contains(git::Status::INDEX_RENAMED) {
		tuple.0 = "R";
	// TODO: Copied in index
	} else if status.contains(git::Status::WT_MODIFIED) {
		tuple.1 = "M";
	} else if status.contains(git::Status::WT_TYPECHANGE) {
		tuple.1 = "T";
	} else if status.contains(git::Status::WT_DELETED) {
		tuple.1 = "D";
	} else if status.contains(git::Status::WT_RENAMED) {
		tuple.1 = "R";
	// TODO: Copied in work tree
	} else if status.contains(git::Status::IGNORED) {
		tuple = ("!", "!");
	} else {
		// Untracked
		tuple = ("?", "?");
	}

	format!("{}{}", tuple.0, tuple.1)
}
