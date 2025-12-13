import { useState } from "react";
import { RecipeCard, RecipeCardProps } from "@/components/RecipeCard";
import { StatsCard } from "@/components/StatsCard";
import { ChefHat, Globe, Tag } from "lucide-react";

interface DashboardProps {
  username: string;
  onRecipeClick?: (id: string) => void;
}

export function Dashboard({ username, onRecipeClick }: DashboardProps) {
  // todo: remove mock functionality
  const mockRecipes: RecipeCardProps[] = [
    {
      id: "1",
      title: "Classic Chocolate Chip Cookies",
      authorName: "You",
      authorInitials: username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase(),
      isPublic: true,
      tags: ["Desserts", "Baking"],
      preview: "2 cups flour, 1 cup butter, 1 cup sugar, 2 eggs...",
    },
    {
      id: "2",
      title: "Spaghetti Carbonara",
      authorName: "You",
      authorInitials: username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase(),
      isPublic: true,
      tags: ["Italian", "Pasta"],
      preview: "400g spaghetti, 200g guanciale, 4 egg yolks...",
    },
    {
      id: "3",
      title: "Grandma's Apple Pie",
      authorName: "You",
      authorInitials: username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase(),
      isPublic: false,
      tags: ["Desserts", "Family"],
      preview: "6 cups sliced apples, 3/4 cup sugar, 2 tbsp flour...",
    },
  ];

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-4xl font-bold">Welcome back, {username.split(" ")[0]}</h1>
        <p className="text-muted-foreground mt-1">Here's what's cooking in your kitchen</p>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <StatsCard
          title="Total Recipes"
          value={24}
          icon={ChefHat}
          description="3 added this week"
        />
        <StatsCard
          title="Public Recipes"
          value={18}
          icon={Globe}
          description="Shared with community"
        />
        <StatsCard
          title="Tags Created"
          value={8}
          icon={Tag}
          description="For organization"
        />
      </div>

      <div className="space-y-4">
        <h2 className="text-xl font-semibold">Recent Recipes</h2>
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {mockRecipes.map((recipe) => (
            <RecipeCard
              key={recipe.id}
              {...recipe}
              onClick={() => onRecipeClick?.(recipe.id)}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
