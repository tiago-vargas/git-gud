use gtk::{gio, prelude::*};
use relm4::prelude::*;

use std::path;

mod log;
mod status;

pub(crate) struct Model {
	content_to_show: Content,
	status: Controller<status::Model>,
	branch_history: Controller<log::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ShowOpenRepoDialog,
	ShowStatus(path::PathBuf),
	ShowLog(path::PathBuf, String),
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
				Content::Status => {
					adw::Bin {
						model.status.widget(),
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
		let status = status::Model::builder()
			.launch(status::Init)
			.detach();
		let branch_history = log::Model::builder()
			.launch(log::Init)
			.detach();
		let model = Self {
			status,
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
									.output(Self::Output::Repository(path.clone()))
									.expect("Receiver should not have been dropped");
								sender.input(Self::Input::ShowStatus(path));
							}
						}
					},
				)
			}
			Self::Input::ShowStatus(path) => {
				let repo = git::Repository::open(path).unwrap();
				let mut options = git::StatusOptions::default();
				options.include_untracked(true);
				let status = repo.statuses(Some(&mut options)).unwrap();

				for entry in status.iter() {
					let file_name = entry.path().expect("File name should be valid UTF-8");
					self.status
						.sender()
						.send(status::Input::AddChangedFileRow(String::from(file_name)))
						.expect("Receiver should not have been dropped");
				}

				self.content_to_show = Content::Status;
			}
			Self::Input::ShowLog(path, branch_name) => {
				self.branch_history
					.sender()
					.send(log::Input::ClearList)
					.expect("Receiver should not have been dropped");

				let repo = git::Repository::open(path)
					.expect("Repo should have been validated in the file-chooser callback");
				let branch = repo
					.find_branch(&branch_name, git::BranchType::Local)
					.or_else(|_| repo.find_branch(&branch_name, git::BranchType::Remote))
					.expect("Branch name should have been gotten from the sidebar");
				let latest_commit = branch
					.get()
					.peel_to_commit()
					.expect("Branch should have a commit");
				let mut revwalk = repo
					.revwalk()
					.expect("Should be able to traverse commit graph");
				revwalk
					.push(latest_commit.id())
					.expect("ID was gotten from `Commit.id()`, so it should work");

				for id in revwalk {
					let id = id.expect("Should be able to iterate over revwalk");
					let commit = repo
						.find_commit(id)
						.expect("ID was gotten from revwalk, so it should work");

					let summary = commit.summary().map(String::from);
					let description = commit.body().map(String::from);
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
	BranchHistory,
	Status,
}

fn is_repository(path: &path::PathBuf) -> bool {
	git::Repository::open(path).is_ok()
}
