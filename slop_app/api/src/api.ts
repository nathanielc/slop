import { DIDSession } from 'did-session'
import { EthereumWebAuth, getAccountId } from '@didtools/pkh-ethereum'
import { ComposeClient } from '@composedb/client'
import { } from '@composedb/types'
import { definition } from './__generated__/definition.js'
import provider from 'eth-provider'

interface BookEntries {
  tag: string,
  entries: BookEntry[]
}
interface BookEntry {
  id: string
  recipeId: string
  recipe?: Recipe
  title: string
  tag: string[]
}

interface Recipe {
  id: string,
  source: string,
  author: Author,
}
interface Author {
  id: string
}

interface BookEntryCreate {
  recipeId: string
  title: string
  tag: string
}

interface RecipeCreate {
  source: string,
}

interface MenuRecipe {
  id: string,
  title: string,
  source: string,
}
interface MenuIngredient {
  name: string,
  amount: string,
}

interface Menu {
  id: string,
  recipes: MenuRecipe[],
  ingredients: MenuIngredient[],
}


interface MenuCreate {
  recipes: MenuRecipe[],
  ingredients: MenuIngredient[],
}


export class Api {
  private composedb: any
  private session: any
  constructor(address: string) {
    console.log('new API', address, definition);
    this.composedb = new ComposeClient({ ceramic: address, definition })
  }

  private async loadSession(authMethod) {
    // Check if user session already in storage
    const sessionStr = localStorage.getItem('didsession')
    let session

    // If session string available, create a new did-session object
    if (sessionStr) {
      session = await DIDSession.fromSession(sessionStr)
    }

    // If no session available, create a new user session and store in local storage
    if (!session || (session.hasSession && session.isExpired)) {
      const session = await DIDSession.authorize(authMethod, { resources: this.composedb.resources })
      localStorage.setItem('didsession', session.serialize())
    }

    return session
  }

  async is_authenticated() {
    let authenticated: boolean
    if (this.session && this.session.hasSession && !this.session.isExpired) {
      authenticated = true
    } else {
      authenticated = false
    }
    console.log('authenticated', authenticated)
    return authenticated
  }
  async authenticate() {
    console.log('authenticate')
    const ethProvider = provider()
    const addresses = await ethProvider.request({ method: 'eth_requestAccounts' })
    const accountId = await getAccountId(ethProvider, addresses[0])
    const authMethod = await EthereumWebAuth.getAuthMethod(ethProvider, accountId)

    this.session = await this.loadSession(authMethod)
    this.composedb.setDID(this.session.did)
  }

  async fetch_my_menu(): Promise<Menu> {
    console.log('fetch_my_menu')
    const result = await this.composedb.executeQuery(`
query BookTags($did: ID!) {
  node(id: $did) {
    ... on CeramicAccount {
      menu {
        id
        recipes {
          id
          title
          source
        }
        ingredients {
          name
          amount
        }
      }
    }
  }
}`,
      {
        did: this.composedb.id,
      })
    console.log('fetch_my_menu', result);

    let menu = result.data.node.menu;
    if (menu === null) {
      menu = {
        id: "",
        recipes: [],
        ingredients: [],
      }
    }
    if (menu.recipes === null) {
      menu.recipes = []
    }
    if (menu.ingredients === null) {
      menu.ingredients = []
    }
    return result.data.node.menu
  }

  async fetch_book_tags(): Promise<Array<string>> {
    console.log('fetch_book_tags')
    const result = await this.composedb.executeQuery(`
query BookTags($did: ID!) {
  node(id: $did) {
    ... on CeramicAccount {
      bookEntryList(first: 100) {
        edges {
          node {
            tag
          }
        }
      }
    }
  }
}`,
      {
        did: this.composedb.id,
      })

    console.log('fetch_book_tags', result);
    const tags = result.data.node.bookEntryList.edges.reduce((tags, edge) => {
      tags[edge.node.tag] = 1;
      return tags
    }, {})
    return Object.keys(tags)
  }

  async fetch_book_entries(tag: string): Promise<BookEntries> {
    console.log('fetch_book_entries', tag)
    const result = await this.composedb.executeQuery(`
query BookEntries($did: ID!, $filter: BookEntryFiltersInput!) {
  node(id: $did) {
    ... on CeramicAccount {
      bookEntryList(first: 100, filters: $filter) {
        edges {
          node {
            id
            recipeId
            title
            tag
          }
        }
      }
    }
  }
}`,
      {
        did: this.composedb.id,
        filter: { where: { tag: { equalTo: tag } } },
      })

    console.log('fetch_book_entries', result);
    const entries = result.data.node.bookEntryList.edges.map((edge) => ({
      id: edge.node.id,
      recipeId: edge.node.recipeId,
      title: edge.node.title,
      tag: edge.node.tag,
    }))
    return { tag, entries }
  }

  async fetch_recipe_x(id: string): Promise<Recipe> {
    console.log('foo');
    return await this.fetch_recipe(id);
  }

  async fetch_recipe(id: string): Promise<Recipe> {
    console.log('fetch_recipe', id)
    const result = await this.composedb.executeQuery(`
query QueryRecipe($id: ID!) {
  node(id: $id) {
    ... on Recipe {
      id
      source
      author {
        id
      }
    }
  }
}`,
      {
        id: id
      }
    )
    console.log('fetch_recipe', result);
    return result.data.node
  }
  async fetch_all_recipes(): Promise<Array<Recipe>> {
    console.log('fetch_all_recipes')
    const result = await this.composedb.executeQuery(`
query {
  recipeIndex(first: 1000) {
    edges {
      node {
        id
        source
        author {
          id
        }
      }
    }
  }
}`)

    console.log('fetch_all_recipes', result);
    const recipes = result.data.recipeIndex.edges.map((edge) => ({
      id: edge.node.id,
      source: edge.node.source,
      author: edge.node.author,
    }))
    return recipes
  }
  async fetch_my_recipes(): Promise<Array<Recipe>> {
    console.log('fetch_my_recipes')
    const result = await this.composedb.executeQuery(`
query MyRecipes($did: ID!) {
  node(id: $did) {
    ... on CeramicAccount {
      recipeList(first: 1000) {
        edges{
          node{
            id
            source
            author {
              id
            }
          }
        }
      }
    }
  }
}`,
      {
        did: this.composedb.id,
      })

    console.log('fetch_my_recipes', result);
    if (result.data.node.recipeList) {
      return result.data.node.recipeList.edges.map((edge) => ({
        id: edge.node.id,
        source: edge.node.source,
        author: edge.node.author,
      }))
    } else {
      return []
    }
  }

  async create_menu(menu: MenuCreate): Promise<string> {
    console.log('create_menu', menu)
    const result = await this.composedb.executeQuery(`
mutation CreateMenu($i: CreateMenuInput!) {
    createMenu(input: $i) {
        document {
            id
        }
    }
}`,
      {
        i: {
          content: menu
        }
      }
    )

    console.log('create_menu', result);
    return result.data.createMenu.document.id;
  }

  async create_recipe(recipe: RecipeCreate): Promise<string> {
    console.log('create_recipe', recipe)
    const result = await this.composedb.executeQuery(`
mutation CreateRecipe($i: CreateRecipeInput!) {
    createRecipe(input: $i) {
        document {
            id
        }
    }
}`,
      {
        i: {
          content: {
            source: recipe.source,
          }
        }
      }
    )

    console.log('create_recipe', result);
    return result.data.createRecipe.document.id;
  }

  async create_book_entry(entry: BookEntryCreate): Promise<string> {
    console.log('create_book_entry', entry)
    const result = await this.composedb.executeQuery(`
mutation CreateBookEntry($i: CreateBookEntryInput!) {
    createBookEntry(input: $i) {
        document {
            id
        }
    }
}`,
      {
        i: {
          content: entry
        }
      }
    )

    console.log('create_book_entry', result);
    return result.data.createBookEntry.document.id;
  }
}