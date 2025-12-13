import { RecipeCard, RecipeCardProps } from "@/components/RecipeCard";
import { EmptyState } from "@/components/EmptyState";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Pencil, Trash2 } from "lucide-react";

interface RecipeBookProps {
  tagName: string;
  username: string;
  onRecipeClick?: (id: string) => void;
  onEditTag?: () => void;
  onDeleteTag?: () => void;
  onBrowseRecipes?: () => void;
}

export function RecipeBook({
  tagName,
  username,
  onRecipeClick,
  onEditTag,
  onDeleteTag,
  onBrowseRecipes,
}: RecipeBookProps) {
  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  // todo: remove mock functionality
  const tagRecipes: Record<string, RecipeCardProps[]> = {
    Desserts: [
      {
        id: "1",
        title: "Classic Chocolate Chip Cookies",
        authorName: "You",
        authorInitials: initials,
        isPublic: true,
        tags: ["Desserts", "Baking"],
        preview: "2 cups flour, 1 cup butter, 1 cup sugar...",
      },
      {
        id: "3",
        title: "Grandma's Apple Pie",
        authorName: "You",
        authorInitials: initials,
        isPublic: false,
        tags: ["Desserts", "Family"],
        preview: "6 cups sliced apples, 3/4 cup sugar...",
      },
    ],
    "Quick Meals": [
      {
        id: "2",
        title: "Spaghetti Carbonara",
        authorName: "You",
        authorInitials: initials,
        isPublic: true,
        tags: ["Italian", "Quick Meals"],
        preview: "400g spaghetti, 200g guanciale...",
      },
    ],
    Italian: [
      {
        id: "2",
        title: "Spaghetti Carbonara",
        authorName: "You",
        authorInitials: initials,
        isPublic: true,
        tags: ["Italian", "Pasta"],
        preview: "400g spaghetti, 200g guanciale...",
      },
      {
        id: "5",
        title: "Homemade Pizza Dough",
        authorName: "You",
        authorInitials: initials,
        isPublic: false,
        tags: ["Italian", "Basics"],
        preview: "3 cups flour, 1 packet yeast...",
      },
    ],
    Healthy: [],
  };

  const recipes = tagRecipes[tagName] || [];

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
          <Button variant="outline" size="icon" onClick={onEditTag} data-testid="button-edit-tag">
            <Pencil className="h-4 w-4" />
          </Button>
          <Button variant="outline" size="icon" onClick={onDeleteTag} data-testid="button-delete-tag">
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {recipes.length === 0 ? (
        <EmptyState type="empty-book" onAction={onBrowseRecipes} />
      ) : (
        <div className="grid gap-4 md:grid-cols-2">
          {recipes.map((recipe) => (
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
