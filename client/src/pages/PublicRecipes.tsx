import { useState, useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { RecipeCard } from "@/components/RecipeCard";
import { SearchBar } from "@/components/SearchBar";
import { EmptyState } from "@/components/EmptyState";
import { useToast } from "@/hooks/use-toast";
import type { Recipe, User } from "@shared/schema";

interface PublicRecipesProps {
  onRecipeClick?: (id: string) => void;
}

type RecipeWithUser = Recipe & { user: User };

function getInitials(user: User): string {
  if (user.firstName && user.lastName) {
    return `${user.firstName[0]}${user.lastName[0]}`.toUpperCase();
  }
  if (user.firstName) {
    return user.firstName.substring(0, 2).toUpperCase();
  }
  if (user.email) {
    return user.email.substring(0, 2).toUpperCase();
  }
  return "U";
}

function getDisplayName(user: User): string {
  if (user.firstName && user.lastName) {
    return `${user.firstName} ${user.lastName}`;
  }
  if (user.firstName) {
    return user.firstName;
  }
  if (user.email) {
    return user.email.split("@")[0];
  }
  return "Anonymous";
}

export function PublicRecipes({ onRecipeClick }: PublicRecipesProps) {
  const { toast } = useToast();
  const [search, setSearch] = useState("");

  const { data: recipes = [], isLoading, error } = useQuery<RecipeWithUser[]>({
    queryKey: ["/api/recipes/public"],
  });

  useEffect(() => {
    if (error) {
      toast({
        title: "Error",
        description: "Failed to load public recipes. Please try again.",
        variant: "destructive",
      });
    }
  }, [error, toast]);

  const filteredRecipes = recipes.filter((r) => {
    if (search && !r.title.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="text-4xl font-bold">Discover Recipes</h1>
        <p className="text-muted-foreground">Explore recipes shared by our community</p>
      </div>

      <SearchBar
        value={search}
        onChange={setSearch}
        onSearch={setSearch}
        placeholder="Search public recipes..."
      />

      {isLoading ? (
        <div className="text-muted-foreground">Loading recipes...</div>
      ) : filteredRecipes.length === 0 ? (
        <EmptyState type={search ? "no-results" : "no-public"} />
      ) : (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {filteredRecipes.map((recipe) => (
            <RecipeCard
              key={recipe.id}
              id={recipe.id}
              title={recipe.title}
              authorName={getDisplayName(recipe.user)}
              authorInitials={getInitials(recipe.user)}
              isPublic={true}
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
