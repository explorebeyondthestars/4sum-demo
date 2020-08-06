tree.pdf : tree.gv
	dot -Tpdf tree.gv -o tree.pdf

clean: 
	rm ./tree.pdf

preview: tree.pdf
	open ./tree.pdf
