single:
	cd main && cargo build --release
	python run.py 1


run:
	cd main && cargo build --release
	python run.py 50

all:
	cd main && cargo build --release
	python run.py 500

bandle:
	python bandle.py
	cat main.rs | xclip -selection clipboard
