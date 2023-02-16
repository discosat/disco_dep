project('csp_to_csh_example', 'rust')

csp_dep = dependency('csp', fallback : ['csp', 'csp_dep'])

# Generate rust binding around c headers
csp_rs = 'csp.rs'
rust = import('unstable-rust')
generated = rust.bindgen(
    input : 'wrapper.h',
    output : csp_rs,
	dependencies: [csp_dep]
)

# Setup cargo
cargo = find_program('cargo', required: true)
cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]

if get_option('profile') == 'release'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif


csp_to_csh_example_lib = custom_target(
  'cargo-build',
  input: generated,
  build_by_default: true,
  build_always_stale: true,
  output: 'lib' + meson.project_name() + '.a',
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
	# Move header files
	'cp', '@INPUT@', meson.project_source_root() / 'src' / csp_rs,
	'&&',
	# Run cargo build
	'env',
    cargo_env,
    cargo, 'build',
	cargo_options,
	'&&',
    'cp', meson.project_source_root() / 'target' / rust_target / 'lib' + meson.project_name() + '.a', '@OUTPUT@',
  ]
)


csp_to_csh_example_inc = include_directories('include')

# Setup link to static/dynamic library
csp_to_csh_example_dep = declare_dependency(
	sources: [csp_to_csh_example_lib[0]],
  include_directories: csp_to_csh_example_inc,
	link_args: ['-lcsp_to_csh_example', '-Llib/csp_to_csh_example'],
)