VALUES=values.dat
DATA=data.dat
PLOT=Rplots.pdf
SRC=src/* src/bin/*

plot: $(PLOT)
	xdg-open "$(PLOT)"

$(PLOT): $(DATA)
	./plot.r

$(DATA): header.txt $(VALUES)
	cat $^ > $@

$(VALUES): $(SRC)
	cargo run --release > $@

clean:
	$(RM) $(VALUES) $(DATA) $(PLOT)

.PHONY: plot
