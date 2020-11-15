#!/usr/bin/Rscript
t <- read.table('data.dat', header=TRUE)
library(ggplot2)
ggplot(t, aes(n, comparisons, color = algorithm)) + ggtitle("# Comparisons") + geom_point() + geom_smooth() + scale_x_log10() + scale_y_log10()
ggplot(t, aes(n, runtime, color = algorithm)) + ggtitle("Runtime (s)") + geom_point() + geom_smooth() + scale_x_log10() + scale_y_log10()
