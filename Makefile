all:
	marp --html --allow-local-files README.md -o public/index.html
	yarn build

init:
	sudo npm install -g @marp-team/marp-cli