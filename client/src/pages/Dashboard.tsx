import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { RecipeCard } from "@/components/RecipeCard";
import { StatsCard } from "@/components/StatsCard";
import { TroughIcon } from "@/components/TroughIcon";
import { Globe } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import type { Recipe } from "@shared/schema";

interface DashboardProps {
  username: string;
  onRecipeClick?: (id: string) => void;
}

export function Dashboard({ username, onRecipeClick }: DashboardProps) {
  const { toast } = useToast();
  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  const { data: stats, error: statsError } = useQuery<{ totalRecipes: number; publicRecipes: number }>({
    queryKey: ["/api/stats"],
  });

  const { data: recentRecipes = [], isLoading, error: recipesError } = useQuery<Recipe[]>({
    queryKey: ["/api/recipes/recent"],
  });

  useEffect(() => {
    if (statsError || recipesError) {
      toast({
        title: "Error",
        description: "Failed to load dashboard data. Please try again.",
        variant: "destructive",
      });
    }
  }, [statsError, recipesError, toast]);

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-4xl font-bold" data-testid="text-welcome">
          Welcome back, {username.split(" ")[0]}
        </h1>
        <p className="text-muted-foreground mt-1">Here's what's cooking in your kitchen</p>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <StatsCard
          title="Total Recipes"
          value={stats?.totalRecipes ?? 0}
          icon={TroughIcon}
          description="In your trough"
        />
        <StatsCard
          title="Public Recipes"
          value={stats?.publicRecipes ?? 0}
          icon={Globe}
          description="Shared with community"
        />
      </div>

      <div className="space-y-4">
        <h2 className="text-xl font-semibold">Recent Recipes</h2>
        {isLoading ? (
          <div className="text-muted-foreground">Loading recipes...</div>
        ) : recentRecipes.length === 0 ? (
          <p className="text-muted-foreground">No recipes yet. Create your first recipe to get started!</p>
        ) : (
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {recentRecipes.map((recipe) => (
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
    </div>
  );
}
