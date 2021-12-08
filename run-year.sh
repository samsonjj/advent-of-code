GREEN='\033[1;32m'
NC='\033[0m'

if [ -z "$1" ]
then
	echo "Provide a year (e.g. 2021)"
	exit
fi

cd $1


ls | sed 's/day-//' | sort -g
the_dirs=$(ls | sed 's/day-//' | sort -g)

for d in $the_dirs; do
	if [[ "$d" =~ ^[0-9]*/?$ ]] ; then
		echo "${GREEN}running project [$1/day-$d] ${NC}"
		cd "day-$d"
		cargo run 2>/dev/null
		echo ""
		cd ..
	fi
done
