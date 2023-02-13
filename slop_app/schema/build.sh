#!/bin/bash

set -e

rm -f *.json *.full.gql

composedb composite:create profile.gql --output profile.json
composedb composite:create recipe.gql --output recipe.json
RECIPE_MODEL=$(jq -r '.models | keys[0]'  recipe.json)
echo $RECIPE_MODEL
cp book_entry.gql book_entry.full.gql
sed -i "s/RECIPE_MODEL/$RECIPE_MODEL/g" book_entry.full.gql
composedb composite:create book_entry.full.gql --output book_entry.json

composedb composite:merge *.json --output composite.json
composedb composite:deploy composite.json
composedb composite:compile composite.json runtime-composite.json
composedb composite:compile composite.json ../api/src/__generated__/definition.js
