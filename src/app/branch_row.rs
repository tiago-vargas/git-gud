use gtk::prelude::*;
use relm4::{factory::FactoryView, prelude::*};

pub(crate) struct Model {
	branch_name: String,
}

pub(crate) struct Init {
	pub(crate) branch_name: String,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();
	type CommandOutput = ();
	type ParentWidget = gtk::ListBox;

	view! {
		gtk::Label {
			set_text: &self.branch_name,
			set_halign: gtk::Align::Start,
		}
	}

	fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
		Self {
			branch_name: init.branch_name,
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
