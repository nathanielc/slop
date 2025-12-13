import { Button } from "@/components/ui/button";
import { BookOpen, ChefHat, Globe, Search } from "lucide-react";

export type EmptyStateType = "no-recipes" | "no-public" | "empty-book" | "no-results";

export interface EmptyStateProps {
  type: EmptyStateType;
  onAction?: () => void;
}

const config: Record<
  EmptyStateType,
  { icon: typeof ChefHat; title: string; description: string; action?: string }
> = {
  "no-recipes": {
    icon: ChefHat,
    title: "No recipes yet",
    description: "Start building your recipe collection by creating your first recipe.",
    action: "Create Recipe",
  },
  "no-public": {
    icon: Globe,
    title: "No public recipes",
    description: "Be the first to share a recipe with the community!",
    action: "Share a Recipe",
  },
  "empty-book": {
    icon: BookOpen,
    title: "This chapter is empty",
    description: "Add recipes to this chapter by tagging them.",
    action: "Browse Recipes",
  },
  "no-results": {
    icon: Search,
    title: "No results found",
    description: "Try adjusting your search or filters.",
  },
};

export function EmptyState({ type, onAction }: EmptyStateProps) {
  const { icon: Icon, title, description, action } = config[type];

  return (
    <div className="flex flex-col items-center justify-center py-16 px-4 text-center">
      <div className="rounded-full bg-muted p-6 mb-6">
        <Icon className="h-12 w-12 text-muted-foreground" />
      </div>
      <h3 className="text-xl font-semibold mb-2">{title}</h3>
      <p className="text-muted-foreground max-w-sm mb-6">{description}</p>
      {action && onAction && (
        <Button onClick={onAction} data-testid="button-empty-action">
          {action}
        </Button>
      )}
    </div>
  );
}
