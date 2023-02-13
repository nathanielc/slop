
.PHONY: run
run: api/dist/api.js
	trunk serve --address 0.0.0.0 --port 4242 --ignore . --no-autoreload

.PHONY: run-graphql
run-graphql: schema/runtime-composite.json
	composedb graphql:server --graphiql schema/runtime-composite.json --port 7777

schema/book_entry.json: schema/book_entry.gql schema/recipe.json
	cp schema/book_entry.gql schema/book_entry.full.gql
	sed -i "s/RECIPE_MODEL/`jq -r '.models | keys[0]'  schema/recipe.json`/g" schema/book_entry.full.gql
	composedb composite:create schema/book_entry.full.gql --output schema/book_entry.json
	rm schema/book_entry.full.gql

schema/profile.json: schema/profile.gql
	composedb composite:create schema/profile.gql --output schema/profile.json

schema/menu.json: schema/menu.gql
	composedb composite:create schema/menu.gql --output schema/menu.json

schema/recipe.json: schema/recipe.gql
	composedb composite:create schema/recipe.gql --output schema/recipe.json

schema/composite.json: schema/profile.json schema/book_entry.json schema/recipe.json schema/menu.json
	composedb composite:merge schema/profile.json schema/book_entry.json schema/recipe.json schema/menu.json --output schema/composite.json
	composedb composite:deploy schema/composite.json

.PHONY: deploy
deploy:
	composedb composite:deploy schema/composite.json

schema/runtime-composite.json: schema/composite.json
	composedb composite:compile schema/composite.json schema/runtime-composite.json

api/src/__generated__/definition.js: schema/composite.json
	composedb composite:compile schema/composite.json api/src/__generated__/definition.js

api/dist/api.js: api/src/api.ts api/src/__generated__/definition.js
	cd api && npm run build


.PHONY: clean
clean:
	@rm -f schema/*.json
	@rm -rf dist/
	@rm -rf api/dist/
	@rm -rf api/src/__generated__
