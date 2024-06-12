use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

mod commit_row;

pub(crate) struct Model {
	commits: FactoryVecDeque<commit_row::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	AddCommitRow(String, String),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		gtk::ScrolledWindow {
			adw::Clamp {
				gtk::Box {
					// This `Box` prevents the list background from being the size of the whole view.
					// It's only noticeable in small enough lists.
					set_orientation: gtk::Orientation::Vertical,

					set_margin_all: 12,

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
				let row = commit_row::Init {
					summary,
					description,
				};
				self.commits.guard().push_back(row);
			}
		}
	}
}
