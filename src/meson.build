pkg_data_dir = get_option('prefix') / get_option('datadir') / meson.project_name()
gnome = import('gnome')

gnome.compile_resources(
	'git-gud',
	'git-gud.gresource.xml',
	gresource_bundle: true,
	install: true,
	install_dir: pkg_data_dir,
)

conf = configuration_data()
conf.set_quoted('APP_ID', application_id)
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('BUILD_TYPE', get_option('buildtype'))
conf.set_quoted('PKG_DATA_DIR', pkg_data_dir)

configure_file(
	input: 'config.rs.in',
	output: 'config.rs',
	configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
	'cp',
	meson.project_build_root() / 'src' / 'config.rs',
	meson.project_source_root() / 'src' / 'config.rs',
	check: true
)

cargo_bin  = find_program('cargo')
cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.project_build_root()  / 'src' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('buildtype') == 'release'
	cargo_opt += [ '--release' ]
	rust_target = 'release'
else
	rust_target = 'debug'
endif

cargo_build = custom_target(
	'cargo-build',
	build_by_default: true,
	build_always_stale: true,
	output: meson.project_name(),
	console: true,
	install: true,
	install_dir: get_option('bindir'),
	command: [
		'env', cargo_env,
		cargo_bin, 'build',
		cargo_opt, '&&', 'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
	]
)
