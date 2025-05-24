"""
This is some quick and dirty code to try to split the generated decoder
into multiple files to improve compilation speed. If it's proven to be worth,
we can look into a more robust solution using `syn`
"""
import os
import shutil
import subprocess


GIT: str = shutil.which("git") #type: ignore
if not GIT:
    print(f"git not found")
    exit(1)
RUSTFMT: str = shutil.which("rustfmt") #type: ignore
if not RUSTFMT:
    print(f"rustfmt not found")
    exit(1)
CARGO: str = shutil.which("cargo") #type: ignore
if not CARGO:
    print(f"cargo not found")
    exit(1)

GIT_REPO = "https://github.com/kromych/disarm64"
MENMONICS_LIST_FILE = "menmonics_all.txt"
MENMONICS_FILE = "menmonics.txt"
DISARM_GEN_OUT_PATH = "target/disarm64_gen/decoder.rs"
DISARM_GEN_ARGS = [
    "-a", "cond"
]
OUTPUT_PATH = "src/decoder"
CHUNK_SIZE = 2000 # number of lines

def main():
    os.makedirs("target/disarm64_gen", exist_ok=True)
    if os.path.exists("disarm64"):
        print("found disarm64, skipping clone")
    else:
        print("cloning disarm64")
        subprocess.run([GIT, "clone", GIT_REPO, "--depth", "1"], check=True)

    print("parsing input menmonics")

    menmonic_matchers = []
    with open(MENMONICS_FILE, "r") as f:
        lines = f.readlines()
        for l in lines:
            l = l.strip()
            if not l or l.startswith("#"):
                continue
            menmonic_matchers.append(make_matcher(l))

    print(f"parsed {len(menmonic_matchers)} menmonics")

    include_menmonics = []
    with open(MENMONICS_LIST_FILE, "r") as f:
        lines = f.readlines()
        for l in lines:
            l = l.strip()
            if not l or l.startswith("#"):
                continue
            for matcher in menmonic_matchers:
                if matcher(l):
                    include_menmonics.append(l)
                    break
            

    print(f"found {len(include_menmonics)} menmonics")
    
    disarm_gen_args = [
        CARGO, "run", "--bin", "disarm64_gen", "--",
        "aarch64.json",
        "-r", "../" + DISARM_GEN_OUT_PATH,
        "-m", ",".join(include_menmonics),
    ] + DISARM_GEN_ARGS
    print(f"running disarm64_gen")
    subprocess.run(disarm_gen_args, cwd="disarm64", check=True)
    print(f"running rustfmt")
    subprocess.run([RUSTFMT, DISARM_GEN_OUT_PATH], check=True)

    if os.path.exists(OUTPUT_PATH):
        print(f"cleaning {OUTPUT_PATH}")
        shutil.rmtree(OUTPUT_PATH)
    os.makedirs(OUTPUT_PATH)

    make_decode_library()
    copy_extra_files()
    run_extra_checks()
  

def make_decode_library():
    print("making decoder library")

    with open(DISARM_GEN_OUT_PATH, "r") as f:
        lines = f.readlines()

    # make the prelude
    prelude = []
    i = 0
    for l in lines:
        # actual stuff
        if l.startswith("#[derive("):
            break
        i+=1
        prelude.append(l)

    # fix the prelude
    prelude = "".join(prelude)
    prelude = prelude.replace("struct Leaf {", "pub(crate) struct Leaf {")
    prelude = prelude.replace("enum Decode {", "pub(crate) enum Decode {")
    prelude = prelude.replace("type DecodeTable = ", "pub(crate) type DecodeTable = ")
    prelude = [prelude]

    # splitting the rest into blocks
    CHUNK_HEADER = """
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::*;
"""
    
    current_chunk_len = 0
    current_chunk_content = CHUNK_HEADER
    had_new_lines = False
    chunks = []

    for l in lines[i:]:
        # skip empty lines
        if l.rstrip() == "":
            continue
        if l.rstrip() == "}":
            # end of a block
            if current_chunk_len > CHUNK_SIZE:
                # extract the chunk
                current_chunk_content += "\n}"
                chunks.append(current_chunk_content)
                current_chunk_content = CHUNK_HEADER
                current_chunk_len = 0
                had_new_lines = False
                continue
        current_chunk_content += l
        current_chunk_len += 1
        had_new_lines = True
    if current_chunk_len > 0 and had_new_lines:
        chunks.append(current_chunk_content)

    # emit each block
    for i, chunk in enumerate(chunks):
        filename = "chunk" + str(i)
        prelude.append(f"mod {filename};\npub use {filename}::*;\n")
        chunk = patch_chunk(chunk)
        emit(os.path.join(OUTPUT_PATH, filename + ".rs"), [chunk])
    # emit the prelude
    emit(os.path.join(OUTPUT_PATH, "mod.rs"), prelude)
        
def patch_chunk(chunk):
    chunk = chunk.replace("fn make_opcode(", "pub(crate) fn make_opcode(")
    return chunk

def emit(filename, lines):
    print(f"emitting {filename}")
    with open(filename, "w", encoding="utf-8") as f:
        for line in lines:
            f.write(line)
    if RUSTFMT is not None:
        subprocess.run([RUSTFMT, filename], check=True)

def make_matcher(l):
    start_star = l.startswith("*")
    end_star = l.endswith("*")
    if start_star and end_star:
        if l == "*":
            return lambda _: True
        l = l[1:-1]
        return lambda x: l in x
    if start_star:
        l = l[1:]
        return lambda x: x.endswith(l)
    if end_star:
        l = l[:-1]
        return lambda x: x.startswith(l)
    return lambda x: x == l

def copy_extra_files():
    print("copying extra files")
    shutil.copyfile("disarm64/LICENSE", f"{OUTPUT_PATH}/LICENSE")
    shutil.copyfile("disarm64/Readme.md", f"{OUTPUT_PATH}/README.md")

def run_extra_checks():
    print("running extra checks")
    subprocess.run([CARGO, "fmt"], check=True)
    # make sure it builds
    subprocess.run([CARGO, "build"], check=True)
    subprocess.run([CARGO, "clippy"], check=True)

if __name__ == "__main__":
    main()
    print("done")
