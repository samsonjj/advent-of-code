session=$1
if [-z "$session"]; then
  echo "please provide a session token"
  exit
fi

year=$2
day=$3

echo "token is $session"

url="https://adventofcode.com/$year/day/$day/input"

echo "downloading from $url ..."

curl --cookie "session=$session" $url > ./src/input.txt
