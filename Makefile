cargo = cargo

crates = storfad

crates_dir="./crates"

default: all
all: check install
check: $(crates)

fmt:
	$(cargo) fmt

storfad:
	$(cargo) clippy -p storfad
	$(cargo) test -p storfad

install:
	$(cargo) install --path $(crates_dir)/storfad --debug --force