# project('rust_example_project', 'rust')

cargo_options = ['--manifest-path', meson.project_source_root() / 'rust_example_project' / 'Cargo.toml']

if get_option('buildtype') == 'release'
	cargo_options += [ '--release' ]
	rust_target = 'release'
	message('Building in release mode')
else
	rust_target = 'debug'
	message('Building in debug mode')
endif

target_dir = meson.project_source_root() / 'target' / rust_target

cargo = find_program('cargo', required: true)
rust_example_project_lib_name = 'lib' + 'rust_example_project' + '.a'
build_command = [
	# Force lib to be build again
	'rm', '-f', target_dir / rust_example_project_lib_name, '&&',
	# Run cargo build
	cargo, 'build', cargo_options, '&&',
	# Copy lib to output folder
	'cp', target_dir / rust_example_project_lib_name, '@OUTPUT@'
]

# Run build command
rust_example_project_lib = custom_target(
	'rust_example_project',
	# Force the bindgen to run
	input: [],
	build_by_default: true,
	build_always_stale: true,
	output: rust_example_project_lib_name,
	console: true,
	install: true,
	install_dir: '..',
	command: build_command
)

# Setup link to static library
rust_example_project_dep = declare_dependency(
	#sources: [rust_example_project_lib[0]],
    # Load the entire library, this ensures that slash sections are loaded properly
	link_whole: [rust_example_project_lib],
    # link_args: ['-Wl,--whole-archive', meson.current_build_dir() / rust_example_project_lib_name, '-Wl,--no-whole-archive'],
    dependencies: [csp_sys_dep, slash_sys_dep]
)

# Build test bed
test(
	'rust_example_project', 
	cargo, 
	args: ['test', '--color=always', cargo_options, '--', '--nocapture'],
	verbose: true,
	timeout: 99999,
	protocol: 'rust'
)