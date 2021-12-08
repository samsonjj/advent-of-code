if [ -z $1 ] ; then
	echo "please provide a year"
	exit 1
fi

if [ -z $2 ] ; then
	echo "please provide a day"
	exit 1
fi

if ! [[ $1 =~ ^[0-9]+ ]] ; then
	echo "year should be a number"
	exit 1
fi

if ! [[ $2 =~ ^[0-9]+ ]] ; then
	echo "day should be a number"
	exit 1
fi

DIR_PATH="$1/day-$2"

if [ -d $DIR_PATH ] ; then
	echo "directory already exists"
	exit 1
fi

# Create dir / copy files
mkdir -p $DIR_PATH
cp -r template/* $DIR_PATH/
cd $DIR_PATH

# Edit Cargo.toml
cat "Cargo.toml" | sed "s/name = \"template\"/name = \"day-$2\"/" >> Cargo.toml.temp
rm Cargo.toml
mv Cargo.toml.temp Cargo.toml

