use adw::prelude::*;
use gtk::glib;
use relm4::{factory::FactoryView, prelude::*};

pub(crate) struct Model {
	summary: String,
	description: Option<String>,
}

pub(crate) struct Init {
	pub(crate) summary: String,
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
		// adw::ExpanderRow {
		// 	set_title: &glib::markup_escape_text(&self.summary),

		// 	set_enable_expansion: self.description.is_some(),
		// 	add_row = &adw::ActionRow {
		// 		set_title?: &self.description.as_ref().map(|o| glib::markup_escape_text(o)),
		// 	},
		// }
		adw::ActionRow {
			set_title: &glib::markup_escape_text(&self.summary),
			set_subtitle?: &self.description.as_ref().map(|o| glib::markup_escape_text(o)),
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
