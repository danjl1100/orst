VALUES=values.dat
DATA=data.dat
PLOT=Rplots.pdf

plot: $(PLOT)
	xdg-open "$(PLOT)"

$(PLOT): $(DATA)
	./plot.r

$(DATA): header.txt $(VALUES)
	cat $^ > $@

$(VALUES):
	cargo run --release > $@

clean:
	$(RM) $(VALUES) $(DATA) $(PLOT)

.PHONY: plot
