use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use super::commit_row;

pub(crate) struct Model {
	commits: FactoryVecDeque<commit_row::Model>,
	class_to_add: Option<String>,
	class_to_remove: Option<String>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	// ListCommits,
	AddCommitRow(String, Option<String>),
	BoxedList,
	NormalList,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		gtk::ScrolledWindow {
		// 	adw::BreakpointBin {
			adw::BreakpointBin {
				set_width_request: 250,
				set_height_request: 50,

				gtk::Box {
					// This prevents the list from being expanded till the bottom.
					set_orientation: gtk::Orientation::Vertical,

					// set_margin_all: 12,
					// VMargin = 36

					#[local_ref]
					commits_list_box -> gtk::ListBox {
						#[watch] add_css_class?: &model.class_to_add,
						#[watch] remove_css_class?: &model.class_to_remove,
					},
				},

				// Applies if width >= 100 px
				add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::parse("min-width: 500px").unwrap()) {

					connect_apply[sender] => move |_this| {
						// commits_list_box.add_css_class("boxed-list");
						sender.input(Input::BoxedList);
						println!("Breakpoint applied");
					},

					connect_unapply[sender] => move |_this| {
						// commits_list_box.remove_css_class("boxed-list");
						sender.input(Input::NormalList);
						println!("Breakpoint unapplied");
					},
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let commits = FactoryVecDeque::builder()
			.launch_default()
			.detach();
		let model = Self {
			commits,
			class_to_add: None,
			class_to_remove: None,
		};

		let commits_list_box = model.commits.widget();
		// let b = adw::Breakpoint::new(adw::BreakpointCondition::parse("min-width: 500px").unwrap());
		// b.add_setter(object, property, value)
		// b.connect_swapped();
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Self::Input::AddCommitRow(summary, description) => {
				self.commits.guard().push_back(commit_row::Init {
					summary,
					description,
				});
			}
			Self::Input::BoxedList => {
				// self.widgets()
				self.class_to_add = Some(String::from("boxed-list"));
				self.class_to_remove = None;
			}
			Self::Input::NormalList => {
				self.class_to_add = None;
				self.class_to_remove = Some(String::from("boxed-list"));
			}
		}
	}
}
