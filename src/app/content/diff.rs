use adw::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model {
	buffer: source::Buffer,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	ShowFakeDiff(git::Oid),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = ();

	view! {
		source::View {
			// set_buffer: Some(&model.buffer),
			// set_show_line_numbers: true,
			// set_highlight_current_line: true,
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			buffer: source::Buffer::new(None),
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Self::Input::ShowFakeDiff(hash) => {
				// self.buffer.set_text("This is a fake text.");
			}
		}
	}
}
