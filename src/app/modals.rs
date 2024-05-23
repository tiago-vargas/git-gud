use adw::prelude::*;
use relm4::prelude::*;

pub(crate) mod about;
pub(crate) mod help;
pub(crate) mod preferences;

#[derive(Debug)]
pub(crate) enum Modal {
	Preferences,
	KeyboardShortcuts(gtk::ShortcutsWindow),
	Help,
	About,
}

pub(crate) fn show_window(modal: Modal) {
	match modal {
		Modal::Preferences => {
			let app = relm4::main_application();
			let main_window = app
				.windows()
				.first()
				.expect("Event should have been triggered by last focused window, thus first item")
				.clone();

			let preferences_window = preferences::Model::builder()
				.transient_for(&main_window)
				.launch(preferences::Init)
				.detach();

			preferences_window.widget().present();
		}
		Modal::KeyboardShortcuts(shortcuts_window) => {
			shortcuts_window.present();
		}
		Modal::Help => {
			let help_window = help::Model::builder()
				.launch(help::Init)
				.detach();
			help_window.widget().present();
		}
		Modal::About => {
			let app = relm4::main_application();
			let main_window = app
				.windows()
				.first()
				.expect("Event should have been triggered by last focused window, thus first item")
				.clone();

			let about_window = about::Model::builder()
				.transient_for(&main_window)
				.launch(about::Init)
				.detach();
			about_window.widget().present();
		}
	}
}
