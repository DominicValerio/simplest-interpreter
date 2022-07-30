#install application in the current directory
# requires cargo
install:
	@if [ -d $(program) ]; then echo "Application already exists" & exit 1; fi
	@echo building
	@cargo build --release 
	@echo "Making folder" named: $(program)
	@mkdir $(program)
	@echo Collecting $(binextension) files in $(builddir)
	@cp $(builddir)/*$(binextension) $(program)
	@echo done
#@echo "$$config" > $(program)/"config.json"
help:
	@echo "make install: builds the project then sets up the app"

program := simple
builddir :=target/release/
binextension :=".exe"
