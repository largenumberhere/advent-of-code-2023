main: build
	./main
	
build: main.asm
	fasm -m 524288 ./main.asm -s main_symbols.fas

debug: build
	# required setting: https://stackoverflow.com/questions/16595417/loading-gdbinit-from-current-directory-fails-with-auto-loading-has-been-declin
	#  one approach is to run `set auto-load safe-path .` inside gdb

	# write the entrypoint address and other settings to .gdbinit file for quicker startup
	./print_entry ./main | xargs -I {} printf "\
	file ./main\n\
	break *{} \nlayout asm\n\
	run\nset disassembly-flavor intel\n\
	" > .gdbinit
	
	# run gdb
	gdb --quiet

clean:
	-rm ./main