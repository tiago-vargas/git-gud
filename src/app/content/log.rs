use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

mod commit_row;

pub(crate) struct Model {
	commits: FactoryVecDeque<commit_row::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ClearList,
	AddCommitRow(Option<String>, Option<String>, git::Oid),
	ShowFakeDiff(usize),
}

#[derive(Debug)]
pub(crate) enum Output {
	ShowFakeDiff(git::Oid),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

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

						connect_row_selected[sender] => move |_, row| {
							if let Some(row) = row {
								sender.input(Self::Input::ShowFakeDiff(row.index() as usize));
							}
						},
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
		let model = Self { commits };

		let commits_list_box = model.commits.widget();
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::ClearList => self.commits.guard().clear(),
			Self::Input::AddCommitRow(summary, description, hash) => {
				let row = commit_row::Init {
					summary,
					description,
					hash
				};
				self.commits.guard().push_back(row);
			}
			Self::Input::ShowFakeDiff(index) => {
				let commit_hash = self
					.commits
					.guard()
					.get(index)
					.expect("Index should've been gotten from a list row")
					.hash;
				sender
					.output(Self::Output::ShowFakeDiff(commit_hash))
					.expect("Receiver should not have been dropped");
			}
		}
	}
}
