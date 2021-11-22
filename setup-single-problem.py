#!/usr/bin/env python3

import json
import os
import re
import sys
import shutil
import subprocess
from pathlib import Path

year = int(sys.argv[1])
day = int(sys.argv[2])
print(f"Setting up single problem for {year}-{day}")

def add_header(src, year, day):
    link_to_file = f"https://github.com/fornwall/advent-of-code/tree/master/crates/core/src/year{year}/day{str(day).rjust(2, '0')}.rs"
    header = f"// Solution to Advent of Code {year}, day {day}: https://adventofcode.com/{year}/day/{day}"
    header += "\n//"
    header += "\n// This is the following file setup for a single problem:"
    header += f"\n// {link_to_file}"

    # Put inlined modules last as they're not relevant compared to the solution:
    suffix = "\n"

    inlined_modules = set()
    pattern = re.compile(r"use (super|crate)::(.*)::(.*?);")
    found = False
    for crate_or_super, module, _ in re.findall(pattern, src):
        if module in inlined_modules:
            continue
        inlined_modules.add(module)

        module_path = module.replace("::", "/")
        if crate_or_super == "super":
            path_in_repo = f"crates/core/src/year{year}/{module_path}.rs"
        else:
            path_in_repo = f"crates/core/src/{module_path}.rs"
        src_to_include = Path(f"{path_in_repo}").read_text().replace('#[cfg(test)]', '')
        module_rust = module.replace("::", " { pub mod ")
        suffix += f"\n\n#[allow(dead_code, unused_imports, unused_macros)]\npub mod {module_rust} {{\n"
        suffix += f"    // This is https://github.com/fornwall/advent-of-code/tree/master/{path_in_repo} inlined to work in the Rust Playground."
        for line in iter(src_to_include.splitlines()):
            if line:
                suffix += f"\n    {line}"
            else:
                suffix += "\n"
        suffix += "\n" + "}" * (1 + module.count("::"))
        found = True

    src = re.sub(r"use super::(.*)?::", lambda match: f"use {match.group(1)}::", src)
    src = re.sub(r"use crate::(.*)?::", lambda match: f"use {match.group(1)}::", src)

    return header + "\n\n" + src + suffix


def replace_include_str(dirpath, src):
    def replace(match):
        included_file = match.group(1)
        replacement_file = os.path.join(dirpath, included_file)
        included_src = Path(replacement_file).read_text()
        included_src = included_src.replace("\\", "\\\\").replace("\n", "\\n").replace('"', '\\"')
        return f'"{included_src}"'
    return re.sub(r'include_str!\("(.*?)"\)', replace, src)

filename = f"crates/core/src/year{year}/day{day:02}.rs"
input_text = Path(f"crates/core/src/year{year}/day{day:02}_input.txt").read_text().replace("\\", "\\\\").replace("\n", "\\n").replace('"', '\\"')
dirpath = f"crates/core/src/year{year}"
print(filename)
src = Path(filename).read_text()

# Strip away use of visualization packages - they just bloat up gist
# with unrelated code and may contain transitive imports:
src = re.sub('^#\\[cfg\\(feature = "visualization"[^;]*;', '', src, flags=re.MULTILINE)

src = add_header(src, year, day)
src = replace_include_str(dirpath, src)

# Finally format source code:
src = subprocess.run(['rustfmt'], stdout=subprocess.PIPE, input=src, encoding='utf-8').stdout

dir_name = f"../generated-single-problem-{year}-{day}"
shutil.rmtree(dir_name, ignore_errors=True)
os.makedirs(f'{dir_name}/src')
os.makedirs(f'{dir_name}/benches')
with open(f'{dir_name}/src/lib.rs', 'w') as f:
    f.write(src)
with open(f'{dir_name}/Cargo.toml', 'w') as f:
    f.write(f"""[package]
name = "advent-of-code"
version = "0.1.0"
edition = "2021"

[dev-dependencies]
criterion = "0.3"
iai = "0.1"

[[bench]]
name = "time"
harness = false

[[bench]]
# See https://bheisler.github.io/criterion.rs/book/iai/getting_started.html
name = "instructions"
harness = false
""")

with open(f"{dir_name}/benches/time.rs", "w") as f:
    f.write(f"""use advent_of_code::solve;
use advent_of_code::input::Input;
use criterion::{{criterion_group, criterion_main, Criterion}};

pub fn criterion_benchmark(c: &mut Criterion) {{
    #![allow(clippy::unwrap_used)]
    let input = "{input_text}";
    c.bench_function("time", |b| {{
        b.iter(|| solve(&mut Input::part_two(input)));
    }});
}}

criterion_group! {{
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .warm_up_time(std::time::Duration::new(1, 0))
        .nresamples(10_000)
        .measurement_time(std::time::Duration::new(3, 0));
    targets = criterion_benchmark
}}

criterion_main!(benches);""")

with open(f"{dir_name}/benches/instructions.rs", "w") as f:
    f.write(f"""use advent_of_code::solve;
use advent_of_code::input::Input;

fn instructions() {{
  let input = "{input_text}";
  solve(&mut Input::part_two(input)).unwrap();
}}
iai::main!(instructions);""")
print(f'Generated {dir_name}')
