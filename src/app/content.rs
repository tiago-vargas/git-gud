use gtk::{gio, prelude::*};
use relm4::prelude::*;

use std::path;

mod log;

pub(crate) struct Model {
	content_to_show: Content,
	branch_history: Controller<log::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ShowOpenRepoDialog,
	IndicateRepositoryWasSelected,
	ShowFakeLog(String),
}

#[derive(Debug)]
pub(crate) enum Output {
	Repository(path::PathBuf),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		adw::Bin {
			match model.content_to_show {
				Content::RepositoryWasSelected => {
					adw::StatusPage {
						set_title: "Stub Page",
						set_description: Some("Repository was selected."),
					}
				}
				Content::NoRepository => {
					adw::StatusPage {
						set_icon_name: Some("folder-symbolic"),
						set_title: "No Repository Selected",

						gtk::CenterBox {
							// `StatusPage` takes 1 child widget, which expands to its width.
							// Having just the button as the child, makes it stretched just too much.
							// Wraping in a `CenterBox` is a workaround to make the button small.
							#[wrap(Some)]
							set_center_widget = &gtk::Button {
								set_label: "Select Repository…",
								add_css_class: "suggested-action",
								add_css_class: "pill",

								connect_clicked[sender] => move |_| {
									sender.input(Self::Input::ShowOpenRepoDialog)
								},
							},
						},
					}
				}
				Content::BranchHistory => {
					adw::Bin {
						model.branch_history.widget(),
					}
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let branch_history = log::Model::builder()
			.launch(log::Init)
			.detach();
		let model = Self {
			content_to_show: Content::NoRepository,
			branch_history,
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::ShowOpenRepoDialog => {
				let app = relm4::main_application();
				let main_window = app
					.windows()
					.first()
					.expect(
						"Event should have been triggered by last focused window, thus first item",
					)
					.clone();

				let home = std::env::var("HOME").expect("System should have set `HOME` on login");
				let dialog = gtk::FileDialog::builder()
					.title("Open Repository")
					.initial_folder(&gio::File::for_path(home))
					.modal(true)
					.build();
				dialog.select_folder(
					Some(&main_window),
					None::<&gio::Cancellable>,
					move |result| {
						if let Ok(selected_folder) = result {
							let path = selected_folder
								.path()
								.expect("Folder was opened via file-chooser, so should have a path");
							if is_repository(&path) {
								sender
									.output(Self::Output::Repository(path))
									.expect("Receiver should not have been dropped");
								sender.input(Self::Input::IndicateRepositoryWasSelected);
							}
						}
					},
				)
			}
			Self::Input::IndicateRepositoryWasSelected => {
				self.content_to_show = Content::RepositoryWasSelected;
			}
			Self::Input::ShowFakeLog(branch) => {
				for i in 1..=3 {
					let summary = format!("Commit {i} of branch {branch}");
					let description = String::from("Blah blah blah.");
					self.branch_history
						.sender()
						.send(log::Input::AddCommitRow(summary, description))
						.expect("Receiver should not have been dropped");
				}

				self.content_to_show = Content::BranchHistory;
			}
		}
	}
}

enum Content {
	NoRepository,
	RepositoryWasSelected,
	BranchHistory,
}

fn is_repository(path: &path::PathBuf) -> bool {
	git::Repository::open(path).is_ok()
}
