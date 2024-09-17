use std::{collections::BTreeMap, fs};
use walkdir::WalkDir;

fn main() {
	let mut rev_acc = BTreeMap::new();
	let mut files = Vec::new();

	for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
		let path = entry.path();
		files.push(path.to_string_lossy().to_string());
		let Ok(contents) = fs::read_to_string(path) else {
			continue;
		};
		let mut lines = contents.lines();
		while let Some(line) = lines.next() {
			if line.trim_start().starts_with("#[revisioned") {
				let Ok(rev): Result<usize, _> = line
					.trim()
					.trim_start_matches("#[revisioned(revision = ")
					.trim_end_matches(")]")
					.parse()
				else {
					continue;
				};
				while let Some(l) = lines.next() {
					// pub, enum ,struct
					if !l.trim_start().starts_with(&['p', 'e', 's']) {
						continue;
					} else {
						let Some(mut name) = l
							.split_whitespace()
							.skip_while(|w| *w != "struct" && *w != "enum")
							.skip_while(|w| *w == "struct" || *w == "enum")
							.next()
						else {
							panic!("foo: {l} {}", path.to_string_lossy())
						};
						if let Some(idx) = name.find('(') {
							name = &name[0..idx];
						}
						rev_acc.insert(format!("{name}({})", path.to_string_lossy()), rev);
						break;
					}
				}
			}
		}
	}

	let lock_str =
		rev_acc.into_iter().map(|(n, r)| format!("{n}={r}")).collect::<Vec<String>>().join("\n");

	fs::write("revision.lock", lock_str).ok();

	if cfg!(target_arch = "wasm32") {
		println!("cargo:rustc-cfg=wasm");
		println!("cargo::rustc-check-cfg=cfg(wasm)");
	}
	if cfg!(any(
		feature = "kv-mem",
		feature = "kv-fdb",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-surrealkv",
		feature = "kv-surrealcs",
	)) {
		println!("cargo:rustc-cfg=storage");
		println!("cargo::rustc-check-cfg=cfg(storage)");
	}
}
