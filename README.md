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
    *1 slice of bread =toast
    *butter #spread
    *jam #spread
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
    *1 banana =peel
    *1 cup blueberries #+
    *1 cup milk #blend
    >


Using `^` with the ingredient operator `*` indicates that the ingredient is a by product of a previous step.
For example using the rendered fat from cooking bacon

    <** Bacon and onions
    * bacon =cook until crispy
    *^rendered fat
    *onions #cook until soft
        #plate
    >

In both cases `+` and `^` do not mean anything to the parser but communicate to the reader of the code the intent.

## Trough and Recipe Cards

Storing recipes as a tree structure allows for visualizing recipes based on their dependent parallel paths instead of simply a linear set of instructions.
The Trough binary will serve a directory structure of `.slop` files and will render the slop files as a recipe card.

The recipe card is a rectangular representation of a tree. It makes it easy to read and understand dependencies between steps of the recipe.


## Inspiration

Slop and Trough are not original ideas, rather a new implementation of old ideas.
Specifically slop the language is heavily inspired by [RxOL](https://web.archive.org/web/20021105191447/http://anthus.com/Recipes/CompCook.html) and Trough's recipe cards are inspired by [Cooking for Engineers](http://www.cookingforengineers.com/).


