use relm4::{
	actions::{AccelsPlus, RelmAction, RelmActionGroup},
	prelude::*,
};

use super::{modals, Model};
use modals::Modal;

relm4::new_action_group!(pub(crate) AppActions, "app");

relm4::new_stateless_action!(pub(crate) ShowPreferences, AppActions, "preferences");
relm4::new_stateless_action!(pub(crate) ShowKeyboardShortcuts, AppActions, "shortcuts");
relm4::new_stateless_action!(pub(crate) ShowHelp, AppActions, "help");
relm4::new_stateless_action!(pub(crate) ShowAbout, AppActions, "about");

impl Model {
	pub(crate) fn create_actions(
		widgets: &<Self as SimpleComponent>::Widgets,
		_sender: &ComponentSender<Self>,
	) {
		let app = relm4::main_adw_application();

		let mut app_actions = RelmActionGroup::<AppActions>::new();

		let show_preferences = RelmAction::<ShowPreferences>::new_stateless(move |_| {
			modals::show_window(Modal::Preferences);
		});
		app.set_accelerators_for_action::<ShowPreferences>(&["<Ctrl>comma"]);
		app_actions.add_action(show_preferences);

		let show_keyboard_shortcuts = {
			let shortcuts_window = widgets.keyboard_shortcuts_window.clone();
			RelmAction::<ShowKeyboardShortcuts>::new_stateless(move |_| {
				modals::show_window(Modal::KeyboardShortcuts(shortcuts_window.clone()));
			})
		};
		app.set_accelerators_for_action::<ShowKeyboardShortcuts>(&["<Ctrl>question"]);
		app_actions.add_action(show_keyboard_shortcuts);

		let show_help = RelmAction::<ShowHelp>::new_stateless(move |_| {
			modals::show_window(Modal::Help);
		});
		app.set_accelerators_for_action::<ShowHelp>(&["F1"]);
		app_actions.add_action(show_help);

		let show_about = RelmAction::<ShowAbout>::new_stateless(move |_| {
			modals::show_window(Modal::About);
		});
		app_actions.add_action(show_about);

		app_actions.register_for_widget(&widgets.main_window);
	}
}
