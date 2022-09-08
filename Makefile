
fmt-os:
	cd os ; cargo fmt; cd ..;

fmt-usr:
	cd usr ; cargo fmt; cd ..;

fmt-all: fmt-os fmt-usr