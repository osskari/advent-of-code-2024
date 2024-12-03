if [[ -z $1 ]]; then
  echo 'Error: Missing parameter'
  return 1
fi

if [[ ! "$1" =~ ^-?[0-9]+$ ]]; then
  echo 'Error: Parameter is not a number'
  return 1
fi

mkdir src/bin/day_"$1"
touch src/bin/day_"$1"/main.rs
touch src/inputs/day"$1".txt

