import { useState } from "react";
import { useQuery, useMutation } from "@tanstack/react-query";
import { useLocation } from "wouter";
import { RecipeView } from "@/components/RecipeView";
import { RecipeEditor } from "@/components/RecipeEditor";
import { Button } from "@/components/ui/button";
import { ArrowLeft } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import { apiRequest, queryClient } from "@/lib/queryClient";
import { useAuth } from "@/hooks/useAuth";
import type { Recipe, Chapter } from "@shared/schema";

interface RecipeDetailProps {
  recipeId: string;
  username: string;
  onBack?: () => void;
}

export function RecipeDetail({ recipeId, username, onBack }: RecipeDetailProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [, setLocation] = useLocation();
  const { toast } = useToast();
  const { user } = useAuth();
  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  const { data, isLoading, error } = useQuery<{ recipe: Recipe; chapters: Chapter[] }>({
    queryKey: ["/api/recipes", recipeId],
  });

  const updateMutation = useMutation({
    mutationFn: async (updateData: { title: string; content: string; tags: string[]; isPublic: boolean }) => {
      const response = await apiRequest("PATCH", `/api/recipes/${recipeId}`, {
        title: updateData.title,
        content: updateData.content,
        isPublic: updateData.isPublic,
        chapters: updateData.tags,
      });
      return response.json();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["/api/recipes", recipeId] });
      queryClient.invalidateQueries({ queryKey: ["/api/recipes"] });
      queryClient.invalidateQueries({ queryKey: ["/api/chapters"] });
      toast({ title: "Recipe updated", description: "Your changes have been saved." });
      setIsEditing(false);
    },
    onError: () => {
      toast({ title: "Error", description: "Failed to update recipe.", variant: "destructive" });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: async () => {
      await apiRequest("DELETE", `/api/recipes/${recipeId}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["/api/recipes"] });
      queryClient.invalidateQueries({ queryKey: ["/api/stats"] });
      queryClient.invalidateQueries({ queryKey: ["/api/chapters"] });
      toast({ title: "Recipe deleted", description: "The recipe has been removed." });
      setLocation("/recipes");
    },
    onError: () => {
      toast({ title: "Error", description: "Failed to delete recipe.", variant: "destructive" });
    },
  });

  if (isLoading) {
    return <div className="text-muted-foreground">Loading recipe...</div>;
  }

  if (error || !data) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={onBack} data-testid="button-back">
          <ArrowLeft className="h-4 w-4 mr-2" />
          Back
        </Button>
        <div className="text-muted-foreground">Recipe not found or access denied.</div>
      </div>
    );
  }

  const { recipe, chapters } = data;
  const isOwner = user?.id === recipe.userId;

  if (isEditing && isOwner) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={() => setIsEditing(false)} data-testid="button-back-to-view">
          <ArrowLeft className="h-4 w-4 mr-2" />
          Back to recipe
        </Button>
        <RecipeEditor
          initialContent={recipe.content}
          initialTags={chapters.map(c => c.name)}
          initialIsPublic={recipe.isPublic}
          onSave={(data) => updateMutation.mutate(data)}
          onCancel={() => setIsEditing(false)}
          onDelete={() => deleteMutation.mutate()}
        />
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <Button variant="ghost" onClick={onBack} data-testid="button-back">
        <ArrowLeft className="h-4 w-4 mr-2" />
        Back
      </Button>
      <RecipeView
        id={recipe.id}
        title={recipe.title}
        authorName={isOwner ? "You" : username}
        authorInitials={initials}
        isPublic={recipe.isPublic}
        tags={chapters.map(c => c.name)}
        content={recipe.content}
        isOwner={isOwner}
        onEdit={() => setIsEditing(true)}
        onAddToBook={() => console.log("Add to book clicked")}
      />
    </div>
  );
}
