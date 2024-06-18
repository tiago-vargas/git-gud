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
				set_label: label_from_status(&self.file_status),
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

fn label_from_status(status: &git::Status) -> &str {
	match *status {
		git::Status::CURRENT => "CURRENT",

		git::Status::INDEX_NEW => "A",
		git::Status::WT_NEW => "U",
		git::Status::INDEX_MODIFIED | git::Status::WT_MODIFIED => "M",
		git::Status::INDEX_DELETED | git::Status::WT_DELETED => "D",
		git::Status::INDEX_RENAMED | git::Status::WT_RENAMED => "R",
		// git::Status::INDEX_TYPECHANGE => "INDEX_TYPECHANGE",
		// git::Status::WT_TYPECHANGE => "WT_TYPECHANGE",

		// git::Status::IGNORED => "IGNORED",
		git::Status::CONFLICTED => "!",

		// other => unimplemented!("Status not listed as `git::Status` variant: {other:?}"),
		other => todo!("Status not supported yet: {other:?}"),
	}
}
