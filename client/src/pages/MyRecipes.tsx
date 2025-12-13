import { useState, useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { RecipeCard } from "@/components/RecipeCard";
import { RecipeFilter, VisibilityFilter, SortOption } from "@/components/RecipeFilter";
import { SearchBar } from "@/components/SearchBar";
import { EmptyState } from "@/components/EmptyState";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import type { Recipe } from "@shared/schema";

interface MyRecipesProps {
  username: string;
  onRecipeClick?: (id: string) => void;
  onNewRecipe?: () => void;
}

export function MyRecipes({ username, onRecipeClick, onNewRecipe }: MyRecipesProps) {
  const { toast } = useToast();
  const [search, setSearch] = useState("");
  const [visibility, setVisibility] = useState<VisibilityFilter>("all");
  const [sort, setSort] = useState<SortOption>("newest");

  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  const { data: recipes = [], isLoading, error } = useQuery<Recipe[]>({
    queryKey: ["/api/recipes"],
  });

  useEffect(() => {
    if (error) {
      toast({
        title: "Error",
        description: "Failed to load recipes. Please try again.",
        variant: "destructive",
      });
    }
  }, [error, toast]);

  const filteredRecipes = recipes
    .filter((r) => {
      if (visibility === "public" && !r.isPublic) return false;
      if (visibility === "private" && r.isPublic) return false;
      if (search && !r.title.toLowerCase().includes(search.toLowerCase())) return false;
      return true;
    })
    .sort((a, b) => {
      if (sort === "title") return a.title.localeCompare(b.title);
      if (sort === "oldest") {
        return new Date(a.createdAt!).getTime() - new Date(b.createdAt!).getTime();
      }
      return new Date(b.updatedAt!).getTime() - new Date(a.updatedAt!).getTime();
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

      {isLoading ? (
        <div className="text-muted-foreground">Loading recipes...</div>
      ) : filteredRecipes.length === 0 ? (
        <EmptyState
          type={search ? "no-results" : "no-recipes"}
          onAction={onNewRecipe}
        />
      ) : (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {filteredRecipes.map((recipe) => (
            <RecipeCard
              key={recipe.id}
              id={recipe.id}
              title={recipe.title}
              authorName="You"
              authorInitials={initials}
              isPublic={recipe.isPublic}
              tags={[]}
              preview={recipe.content.substring(0, 100) + "..."}
              onClick={() => onRecipeClick?.(recipe.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
