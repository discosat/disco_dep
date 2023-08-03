
# Generate rust binding around slash header files
slash_dep = dependency('slash', fallback : ['slash', 'slash_dep'])
slash_rs = 'slash_sys.rs'
rust = import('unstable-rust')

slash_sys_bind = rust.bindgen(
    input : 'slash.h',
    output : slash_rs,
	dependencies: [slash_dep]
)

slash_sys_lib = custom_target(
	'slash_sys',
	# Force the bindgen to run
	depends: slash_sys_bind,
	build_by_default: true,
	build_always_stale: true,
	output: 'slash_sys_empty',
	console: true,
	command: [
        'cp', meson.current_build_dir() / slash_rs, meson.current_source_dir() / 'src' / slash_rs
    ]
)

slash_sys_dep = declare_dependency(
	sources: [slash_sys_lib]
)

cargo = find_program('cargo', required: true)
cargo_options = ['--manifest-path', meson.project_source_root() / 'slash_sys' / 'Cargo.toml']

if get_option('buildtype') == 'release'
	cargo_options += [ '--release' ]
	message('Building in release mode')
else
	message('Building in debug mode')
endif
test(
	'slash_sys', 
	cargo, 
	args: ['test', '--color=always', cargo_options, '--', '--nocapture'],
	verbose: true,
	timeout: 9999,
	protocol: 'rust'
)