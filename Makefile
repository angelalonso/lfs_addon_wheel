# based on https://gist.github.com/jlgerber/0f280236c2ee1b741dfe41a38d39a467
prog :=lfs_addon_wheel

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

all: build start_arduino run
 
build:
	@cargo clippy
	@cargo build $(release)

run:
	@sudo target/$(target)/$(prog)

start_arduino:
	@arduino &
	@sleep 2
	@clear

install:
	@cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

help:
	@echo "usage: make $(prog) [debug=1]"

.PHONY: all
