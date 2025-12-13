import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { RecipeCard } from "@/components/RecipeCard";
import { EmptyState } from "@/components/EmptyState";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Pencil, Trash2 } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import type { Recipe } from "@shared/schema";

interface RecipeBookProps {
  tagName: string;
  username: string;
  onRecipeClick?: (id: string) => void;
  onEditChapter?: () => void;
  onDeleteChapter?: () => void;
  onBrowseRecipes?: () => void;
}

export function RecipeBook({
  tagName,
  username,
  onRecipeClick,
  onEditChapter,
  onDeleteChapter,
  onBrowseRecipes,
}: RecipeBookProps) {
  const { toast } = useToast();
  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  const { data: recipes = [], isLoading, error } = useQuery<Recipe[]>({
    queryKey: [`/api/chapters/${encodeURIComponent(tagName)}/recipes`],
  });

  useEffect(() => {
    if (error) {
      toast({
        title: "Error",
        description: "Failed to load chapter recipes. Please try again.",
        variant: "destructive",
      });
    }
  }, [error, toast]);

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <h1 className="text-4xl font-bold font-serif">{tagName}</h1>
          <Badge variant="secondary" className="text-sm">
            {recipes.length} recipe{recipes.length !== 1 ? "s" : ""}
          </Badge>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="icon" onClick={onEditChapter} data-testid="button-edit-chapter">
            <Pencil className="h-4 w-4" />
          </Button>
          <Button variant="outline" size="icon" onClick={onDeleteChapter} data-testid="button-delete-chapter">
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {isLoading ? (
        <div className="text-muted-foreground">Loading recipes...</div>
      ) : recipes.length === 0 ? (
        <EmptyState type="empty-book" onAction={onBrowseRecipes} />
      ) : (
        <div className="grid gap-4 md:grid-cols-2">
          {recipes.map((recipe) => (
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
