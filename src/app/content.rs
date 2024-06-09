use adw::{gio, prelude::*};
use relm4::prelude::*;

use std::path;

use crate::app::log;

pub(crate) struct Model {
	repository_was_selected: bool,
	should_show_log: bool,
	log: Controller<log::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ShowOpenRepoDialog,
	IndicateRepositoryWasSelected,
	ShowLog(gio::File, String),
}

#[derive(Debug)]
pub(crate) enum Output {
	Repository(gio::File),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		adw::Bin {
			if model.repository_was_selected {
				adw::Bin {
					if model.should_show_log {
						adw::Bin {
							model.log.widget(),
						}
					} else {
						adw::StatusPage {
							set_title: "Stub Page",
							set_description: Some("Repository was selected."),
						}
					}
				}
			} else {
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
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let log = log::Model::builder()
			.launch(log::Init)
			.detach();
		let model = Self {
			repository_was_selected: false,
			should_show_log: false,
			log,
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
									.output(Self::Output::Repository(selected_folder.clone()))
									.expect("Receiver should not have been dropped");
								sender.input(Self::Input::IndicateRepositoryWasSelected);
							}
						}
					},
				)
			}
			Self::Input::IndicateRepositoryWasSelected => {
				self.repository_was_selected = true;
			}
			Self::Input::ShowLog(repo, branch) => {
				let path = repo
					.path()
					.expect("Folder was opened via file-chooser, so should have a path");
				let repo = git::Repository::open(path).unwrap();
				let dev = repo.find_branch(&branch, git::BranchType::Local).unwrap();
				let latest_commit = dev.get().peel_to_commit().unwrap();
				let mut revwalker = repo.revwalk().unwrap();
				revwalker.push(latest_commit.id()).unwrap();

				for id in revwalker {
					let id = id.unwrap();
					let commit = repo.find_commit(id).unwrap();

					self.log
					.sender()
					.send(log::Input::AddCommitRow(
						commit.summary().as_ref().unwrap().to_string(),
						commit.body().map(String::from),
					))
					.unwrap();
				}


				self.should_show_log = true;
			}
		}
	}
}

fn is_repository(path: &path::PathBuf) -> bool {
	git::Repository::open(path).is_ok()
}
