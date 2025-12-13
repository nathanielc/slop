import { useState } from "react";
import { RecipeCard, RecipeCardProps } from "@/components/RecipeCard";
import { RecipeFilter, VisibilityFilter, SortOption } from "@/components/RecipeFilter";
import { SearchBar } from "@/components/SearchBar";
import { EmptyState } from "@/components/EmptyState";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";

interface MyRecipesProps {
  username: string;
  onRecipeClick?: (id: string) => void;
  onNewRecipe?: () => void;
}

export function MyRecipes({ username, onRecipeClick, onNewRecipe }: MyRecipesProps) {
  const [search, setSearch] = useState("");
  const [visibility, setVisibility] = useState<VisibilityFilter>("all");
  const [sort, setSort] = useState<SortOption>("newest");

  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  // todo: remove mock functionality
  const mockRecipes: RecipeCardProps[] = [
    {
      id: "1",
      title: "Classic Chocolate Chip Cookies",
      authorName: "You",
      authorInitials: initials,
      isPublic: true,
      tags: ["Desserts", "Baking", "Quick"],
      preview: "2 cups flour, 1 cup butter, 1 cup sugar, 2 eggs, 1 tsp vanilla...",
    },
    {
      id: "2",
      title: "Spaghetti Carbonara",
      authorName: "You",
      authorInitials: initials,
      isPublic: true,
      tags: ["Italian", "Pasta", "Quick Meals"],
      preview: "400g spaghetti, 200g guanciale, 4 egg yolks, Pecorino Romano...",
    },
    {
      id: "3",
      title: "Grandma's Apple Pie",
      authorName: "You",
      authorInitials: initials,
      isPublic: false,
      tags: ["Desserts", "Family", "Baking"],
      preview: "6 cups sliced apples, 3/4 cup sugar, 2 tbsp flour, cinnamon...",
    },
    {
      id: "4",
      title: "Thai Green Curry",
      authorName: "You",
      authorInitials: initials,
      isPublic: true,
      tags: ["Thai", "Curry", "Spicy"],
      preview: "Green curry paste, coconut milk, chicken, bamboo shoots, basil...",
    },
    {
      id: "5",
      title: "Homemade Pizza Dough",
      authorName: "You",
      authorInitials: initials,
      isPublic: false,
      tags: ["Italian", "Basics"],
      preview: "3 cups flour, 1 packet yeast, 1 cup warm water, 2 tbsp olive oil...",
    },
    {
      id: "6",
      title: "Chicken Tikka Masala",
      authorName: "You",
      authorInitials: initials,
      isPublic: true,
      tags: ["Indian", "Curry", "Comfort Food"],
      preview: "Chicken thighs, yogurt marinade, tomato sauce, garam masala...",
    },
  ];

  const filteredRecipes = mockRecipes
    .filter((r) => {
      if (visibility === "public" && !r.isPublic) return false;
      if (visibility === "private" && r.isPublic) return false;
      if (search && !r.title.toLowerCase().includes(search.toLowerCase())) return false;
      return true;
    })
    .sort((a, b) => {
      if (sort === "title") return a.title.localeCompare(b.title);
      if (sort === "oldest") return 1;
      return -1;
    });

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-center justify-between gap-4">
        <h1 className="text-4xl font-bold">My Recipes</h1>
        <Button onClick={onNewRecipe} data-testid="button-new-recipe-page">
          <Plus className="h-4 w-4 mr-2" />
          New Recipe
        </Button>
      </div>

      <div className="flex flex-wrap items-center gap-4">
        <SearchBar
          value={search}
          onChange={setSearch}
          onSearch={setSearch}
        />
        <RecipeFilter
          visibility={visibility}
          sort={sort}
          onVisibilityChange={setVisibility}
          onSortChange={setSort}
        />
      </div>

      {filteredRecipes.length === 0 ? (
        <EmptyState
          type={search ? "no-results" : "no-recipes"}
          onAction={onNewRecipe}
        />
      ) : (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {filteredRecipes.map((recipe) => (
            <RecipeCard
              key={recipe.id}
              {...recipe}
              onClick={() => onRecipeClick?.(recipe.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
