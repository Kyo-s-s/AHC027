run:
	cd main && cargo build --release
	python run.py

bandle:
	python bandle.py
	cat main.rs | xclip -selection clipboard