use adw::prelude::*;
use gtk::glib;
use relm4::{factory::FactoryView, prelude::*};

pub(crate) struct Model {
	summary: Option<String>,
	description: Option<String>,
}

pub(crate) struct Init {
	pub(crate) summary: Option<String>,
	pub(crate) description: Option<String>,
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
			set_title?: &self.summary.as_ref().map(|s| glib::markup_escape_text(s)),
			set_subtitle?: &self.description.as_ref().map(|d| glib::markup_escape_text(d)),
		}
	}

	fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
		Self {
			summary: init.summary,
			description: init.description,
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

	fn update(&mut self, input: Self::Input, _sender: FactorySender<Self>) {
		let () = input;
	}
}
