# Slop

Slop is a programming language to store recipes.
Slop uses postfix notation to create a tree of ingredients and instructions for creating a recipe.

This repo publishes two binaries:

* Trough - HTTP Server to visualize recipes
* slop-language-server - A server implementation of the Language Server Protocol to make writing recipes fast and accurate.

## Slop Language

The Slop language follows a few simple rules.
A recipe is inclused in `<` and `>` and there are three primary operators: `*` for marking ingredients with `=` and `#` which are unary and binary operators respectively.

The `*` is a prefix operator, meaning it starts a new ingredient while `=` and `#` are postfix operators, meaning they follow their operands.

The unary operator `=` applies to a single operand and the binray `#` operator applies to the procedding two operators.
Each of these operators can contain text following the operator.

Here is a simple example recipe for toast and jam.

    <
    *1 slice: bread =toast
    *1/2 tsp: butter #spread
    *1 tbsp: jam #spread
    >

There are a few other meta data related operators:

* `**` - Marks the title of a recipe
* `##` - Preamble steps that don't involve ingredients
* `#*` - Final comments, to record a description of yield and estimated time

A complete grammar can be found in the source code.


### Conventions

Beyond the langauge specification there are two conventions that if followed can make writing and reading recipes easier.

Using `+` as the only text for a `#` operator indicates that multiple operands should be combined into a single step.
For example in the below smoothie recipe it reads as adding all ingredients into the blender before blending.

    <**Smoothie
    *1: banana =peel
    *1 cup: blueberries #+
    *1 cup: milk #blend
    >


Using `^` with the ingredient operator `*` indicates that the ingredient is a by product of a previous step.
For example using the rendered fat from cooking bacon

    <** Bacon and onions
    *1 lbs: bacon =cook until crispy =set aside
    *^rendered fat
    *1: yellow onion #cook until soft
        #plate
    >

In both cases `+` and `^` do not mean anything to the parser but communicate to the reader of the code the intent.

## Sharing Recipes

Slop comes with both a web application and a server to host and share recipes.
The app is built on the [Ceramic Network](https://ceramic.network/) which means that you can share recipes with anyone also running the app.
The Ceramic Network enables each individual hosted server to synchronize the global set of recipes.

The web application itself is fully client side rendered meaning that all you need is to host a Ceramic node and change the `Server Address` in `Settings`.
You can find the application hosted at https://nathanielc.github.io/slop/ .

### Participate in the Slop network

Follow the steps to [run a Ceramic node in production](https://developers.ceramic.network/run/nodes/nodes/).

Once you have a Ceramic + ComposeDB node running follow these steps to tell the node about the Slop data.

First ensure your `daemon.config.json` has these `indexing` settings:

```json
  "indexing": {
    "allow-queries-before-historical-sync": true,
    "disable-composedb": false,
    "enable-historical-sync": true
  }
```


Then run these command to instruct the Ceramic daemon to synchronize the Slop data.

    $ wget https://raw.githubusercontent.com/nathanielc/slop/master/app/schema/composite.json
    $ composedb composite:deploy composite.json # Make sure you have configured your Admin DID for this command to work


Now you should be able to immediately use the web application to create recipes, build menus and organize your recipe book.
It may take some time for the global recipes to synchronize to your server before you can explore recipes from others.

## Example

The following is an example of a relatively envolved recipie that makes use of the various features of Slop.

    <** Souffle pancake with one egg
    *3 or 4 drops: lemon juice
    *1 : egg =separate keep white #stir in =beat at medium speed, until foamy
    *1 1/2 tbsp: sugar #sprinkle in =beat at medium speed 3m until firm peaks form
    *^egg yolk
    *2 tbsp: flour #+
    *1 tbsp: milk #mix to combine
    *1/2 tsp: vanilla #stir in
    *^1/3 of: egg white mixture #mix with circular motion
        #fold in with flat spatula
    *1 tsp: oil =heat in pan 1m
    *^2/3 of: pancake mixture #scoop into pan as two pancakes
    *2 tsp: water #add to sides of pan =cover cook 2m on medium heat
    *1 tsp: water #add to sides of pan #place on top
        =cover cook 5m on medium low heat
        =flip
        =cover cook 5m
        =serve with fruit and syrup/powdered sugar
    #* Makes 2 pancakes
    >



![Souffle Pancake Recipe Card](./souffle_pancake.svg)

## Inspiration

Slop and Trough are not original ideas, rather a new implementation of old ideas.
Specifically slop the language is heavily inspired by [RxOL](https://web.archive.org/web/20021105191447/http://anthus.com/Recipes/CompCook.html) and Trough's recipe cards are inspired by [Cooking for Engineers](http://www.cookingforengineers.com/).


