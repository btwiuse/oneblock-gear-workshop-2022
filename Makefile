all:
	marp --html --allow-local-files README.md -o public/index.html
	yarn build
