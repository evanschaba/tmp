#!/bin/zsh

# remove tmp file containing all the shell cmd output
rm tmp; 

# recreate tmp file
touch tmp;

# compile src & dump output to tmp outputfile
rustc ft_putchr.rs &> tmp;  

# append ascii divider symbol to tmp file
echo "----------------------------------------------------------------------------------------------------------------- exec ----------------------------------------------------------------------------------------------------------------- \n" &>> tmp;

# exec bin & append execution output to tmp outputfile
./ft_putchr &>> tmp; 

# copy tmp outputfile contents to clipboard
head -n 1000 tmp | pbcopy;

# print output file using
cat tmp; # aliased (bat -- prettier package for printing output that's better than cat)
# head -n 10000 tmp; (print first 10 000 lines of a file along with its file name prefixed at the top)