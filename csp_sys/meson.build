# Generate rust binding around csp header files
csp_dep = dependency('csp', fallback : ['csp', 'csp_dep'])
csp_rs = 'csp_sys.rs'
rust = import('unstable-rust')

csp_sys_bind = rust.bindgen(
    input : 'csp.h',
    output : csp_rs,
	dependencies: [csp_dep]
)

csp_sys_lib = custom_target(
	'csp_sys',
	# Force the bindgen to run
	input: csp_sys_bind,
	build_by_default: true,
	build_always_stale: true,
	output: 'csp_sys_empty',
	console: true,
	command: [
        'cp', meson.current_build_dir() / csp_rs, meson.current_source_dir() / 'src' / csp_rs
    ]
)

csp_sys_dep = declare_dependency(
	sources: [csp_sys_lib]
)