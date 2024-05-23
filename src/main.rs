use relm4::prelude::*;

mod app;
mod config;

use config::{APP_ID, PKG_DATA_DIR};
use gtk::gio;

fn main() {
	// Load resources
	let resources = gio::Resource::load(PKG_DATA_DIR.to_owned() + "/git-gud.gresource")
		.expect("Should be able to load resources");
	gio::resources_register(&resources);

	// Run the app
	let app = RelmApp::new(APP_ID);
	app.run::<app::Model>(app::Init);
}
