use adw::prelude::*;
use gtk::gio;
use relm4::prelude::*;

use std::{ffi, path};

use crate::config::BUILD_TYPE;

mod actions;
mod content;
mod modals;
mod settings;

pub(crate) struct Model {
	header_bar_subtitle: ffi::OsString,
	repository: Option<gio::File>,
	content: Controller<content::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	SetRepository(gio::File),
	SetHeaderBarSubtitle(path::PathBuf),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	menu! {
		primary_menu: {
			section! {
				"Preferences" => actions::ShowPreferences,
				"Keyboard Shortcuts" => actions::ShowKeyboardShortcuts,
				"Help" => actions::ShowHelp,
				"About App" => actions::ShowAbout,
			},
		}
	}

	view! {
		main_window = adw::ApplicationWindow {
			set_title: Some("Git Gud"),

			add_css_class?: if BUILD_TYPE == "debug" { Some("devel") } else { None },

			#[wrap(Some)]
			set_help_overlay: keyboard_shortcuts_window =
				&gtk::Builder::from_resource(
					"/com/github/tiago_vargas/git_gud/app/modals/keyboard-shortcuts-window.ui"
				)
				.object::<gtk::ShortcutsWindow>("help_overlay")
				.unwrap() -> gtk::ShortcutsWindow {
					set_transient_for: Some(&main_window),  // Apparently, this isn't necessary
				},

			adw::OverlaySplitView {
				#[watch] set_show_sidebar: model.repository.is_some(),

				#[wrap(Some)]
				set_sidebar = &adw::NavigationPage {
					set_title: "Branches",

					adw::ToolbarView {
						add_top_bar = &adw::HeaderBar {
							pack_end = &gtk::MenuButton {
								set_icon_name: "open-menu-symbolic",
								set_menu_model: Some(&primary_menu),
							},
						},

						adw::StatusPage {
							set_title: "Sidebar",
						},
					},
				},

				adw::NavigationPage {
					set_title: "Content",

					adw::ToolbarView {
						add_top_bar = &adw::HeaderBar {
							#[wrap(Some)]
							set_title_widget = &adw::WindowTitle {
								set_title: "Git Gud",
								#[watch] set_subtitle?: model.header_bar_subtitle.to_str(),
							},
						},

						#[wrap(Some)]
						set_content = model.content.widget(),
					},
				}
			},
		}
	}

	fn init(
		_init: Self::Init,
		window: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let content = content::Model::builder()
			.launch(content::Init)
			.forward(sender.input_sender(), |output| match output {
				content::Output::Repository(folder) => Self::Input::SetRepository(folder),
			});
		let placeholder_subtitle = ffi::OsString::default();
		let model = Model {
			header_bar_subtitle: placeholder_subtitle,
			repository: None,
			content,
		};

		let widgets = view_output!();

		Self::load_window_state(&widgets);
		Self::create_actions(&widgets, &sender);

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::SetRepository(folder) => {
				let path = folder
					.path()
					.expect("Folder was opened via file-chooser, so should have a path");
				sender.input(Self::Input::SetHeaderBarSubtitle(path));
				self.repository = Some(folder);
			}
			Self::Input::SetHeaderBarSubtitle(path) => {
				self.header_bar_subtitle = path
					.file_name()
					.expect("Chosen repo should have a name")
					.to_os_string()
			}
		}
	}

	fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
		Self::save_window_state(widgets);
	}
}
