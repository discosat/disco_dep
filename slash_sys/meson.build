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
	input: slash_sys_bind,
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