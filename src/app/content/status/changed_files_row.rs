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

			add_suffix = &gtk::Image {
				set_from_icon_name: Some(&icon_name_from_status(&self.file_status)),
				// set_from_resource: Some(&icon_name_from_status(&self.file_status)),

				// DEBUG
				connect_map => move |this| {
					let icon_name = this.icon_name();
					println!("ICON: {icon_name:?}");
					println!();
				},
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

fn icon_name_from_status(status: &git::Status) -> String {
	// let prefix = String::from("/com/github/tiago_vargas/git_gud/icons/hicolor/symbolic/actions/");
	let prefix = String::from("com.github.tiago_vargas.git_gud-");
	// let prefix = String::from("");
	// let suffix = ".svg";
	let suffix = "";

	println!("STATUS: {status:?}");

	match *status {
		git::Status::INDEX_NEW => prefix + "index-new-symbolic" + suffix,
		git::Status::INDEX_MODIFIED => prefix + "wt-modified-symbolic" + suffix,  // TEMP
		// git::Status::INDEX_DELETED => "INDEX_DELETED",
		// git::Status::INDEX_RENAMED => "INDEX_RENAMED",

		// WORKS!
		git::Status::WT_NEW => prefix + "wt-new-symbolic" + suffix,
		// git::Status::WT_NEW => "wt-new-symbolic",
		git::Status::WT_MODIFIED => prefix + "wt-modified-symbolic" + suffix,
		git::Status::WT_DELETED => prefix + "wt-deleted-symbolic" + suffix,
		// git::Status::WT_TYPECHANGE => "WT_TYPECHANGE",
		git::Status::WT_RENAMED => prefix + "wt-renamed-symbolic" + suffix,

		git::Status::CONFLICTED => prefix + "conflicted-symbolic" + suffix,

		// other => todo!("No icon for {other:#?} yet"),
		_other => prefix + "conflicted-symbolic" + suffix,  // TEMP
	}
}
