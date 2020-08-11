output.gv:
	cargo run

output.pdf : output.gv
	dot -Tpdf output.gv -o output.pdf

preview: output.pdf
	open output.pdf

png: output.gv
	dot -Tpng output.gv -o output.png

target/debug/tree: 
	cargo build

test: target/debug/tree
	./target/debug/tree 1>output.log.txt

clean:
	rm -f output.pdf
	rm -f output.png
	rm -f output.gv
	rm -f output.log.txt