use adw::prelude::*;
use gtk::glib;
use relm4::{factory::FactoryView, prelude::*};

pub(crate) struct Model {
	summary: Option<String>,
	description: Option<String>,
	pub(crate) hash: git::Oid,
}

pub(crate) struct Init {
	pub(crate) summary: Option<String>,
	pub(crate) description: Option<String>,
	pub(crate) hash: git::Oid,
}

#[derive(Debug)]
pub(crate) enum Input {
	PrintHash,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();
	type CommandOutput = ();
	type ParentWidget = gtk::ListBox;

	view! {
		adw::ExpanderRow {
			set_title?: &self.summary.as_ref().map(|s| glib::markup_escape_text(s)),
			set_subtitle?: &self.description.as_ref().map(|d| glib::markup_escape_text(d)),

			add_suffix = &gtk::Button {
				set_icon_name: "copy-symbolic",
				// TODO: Add top margin
				// This button is right at the top
				set_valign: gtk::Align::Start,

				connect_clicked[sender] => move |_| {
					sender.input(Self::Input::PrintHash);
				},
			},

			add_row = &gtk::Label {
				// set_markup: Some(&format!("<b>{}</b>", self.hash)),
				// set_xalign: 0.0,
			},
		}
	}

	fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
		Self {
			summary: init.summary,
			description: init.description,
			hash: init.hash,
		}
	}

	fn init_widgets(
		&mut self,
		_index: &Self::Index,
		root: Self::Root,
		_returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		sender: FactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, input: Self::Input, _sender: FactorySender<Self>) {
		match input {
			Self::Input::PrintHash => println!("Hash: {}", self.hash),
		}
	}
}
