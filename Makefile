
tailwind-watch:
	npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

dioxus-watch:
	dx serve 

build:
	rm -rf ./docs
	dx build --release
	cp ./docs/index.html ./docs/404.html