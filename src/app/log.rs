use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use super::commit_row;

pub(crate) struct Model {
	commits: FactoryVecDeque<commit_row::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	// ListCommits,
	AddCommitRow(String, Option<String>),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		gtk::ScrolledWindow {
		// 	adw::BreakpointBin {
			adw::Clamp {
				gtk::Box {
					// This prevents the list from being expanded till the bottom.
					set_orientation: gtk::Orientation::Vertical,

					// set_margin_all: 12,
					// VMargin = 36

					#[local_ref]
					commits_list_box -> gtk::ListBox {
						add_css_class: "boxed-list",
					},
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let commits = FactoryVecDeque::builder()
			.launch_default()
			.detach();
		let model = Self { commits };

		let commits_list_box = model.commits.widget();
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
		}
	}
}
