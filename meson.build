project('csp_to_csh_example', 'rust')

cargo_options = []
generated_headers = []
build_command = []

# Generate rust binding around csp header files
csp_dep = dependency('csp', fallback : ['csp', 'csp_dep'])
csp_rs = 'csp_sys.rs'
rust = import('unstable-rust')

generated_headers += rust.bindgen(
    input : 'wrapper.h',
    output : csp_rs,
	dependencies: [csp_dep]
)
build_command += ['cp', meson.project_build_root() / csp_rs, meson.project_source_root() / 'src' / 'sys' / csp_rs, '&&']

# Generate rust binding around slash header files
slash_dep = dependency('slash', fallback : ['slash', 'slash_dep'], required: true)
slash_rs = 'slash_sys.rs'

generated_headers += rust.bindgen(
	input : 'bindgen_headers/slash.h',
	output : slash_rs,
	dependencies: [slash_dep]
)
build_command += ['cp', meson.project_build_root() / slash_rs, meson.project_source_root() / 'src' / 'sys' / slash_rs, '&&']

# Setup cargo
cargo = find_program('cargo', required: true)
cargo_options += [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]

if get_option('buildtype') == 'release'
	cargo_options += [ '--release' ]
	rust_target = 'release'
	message('Building in release mode')
else
	rust_target = 'debug'
	message('Building in debug mode')
endif

target_dir = meson.project_source_root() / 'target' / rust_target
build_command += [
	# Force lib to be build again
	'rm', '-f', target_dir / 'lib' + meson.project_name() + '.a', '&&',
	# Run cargo build
	cargo, 'build', cargo_options, '&&',
	# Copy lib to output folder
	'cp', target_dir / 'lib' + meson.project_name() + '.a', '@OUTPUT@'
]

# Run build command
csp_to_csh_example_lib = custom_target(
	'cargo-build',
	# Force the bindgen to run
	input: generated_headers,
	build_by_default: true,
	build_always_stale: true,
	output: 'lib' + meson.project_name() + '.a',
	console: true,
	install: true,
	install_dir: get_option('bindir'),
	command: build_command
)

csp_to_csh_example_inc = include_directories('include')

# Setup link to static library
csp_to_csh_example_dep = declare_dependency(
	sources: [csp_to_csh_example_lib[0]],
	include_directories: csp_to_csh_example_inc,
	link_args: ['-lcsp_to_csh_example', '-Llib/csp_to_csh_example']
)

# Build test bed
test(
	'csp_to_csh_example_test', 
	cargo, 
	args: ['test', '--color=always', cargo_options, '--', '--nocapture'],
	verbose: true,
	timeout: 99999,
	protocol: 'rust'
)