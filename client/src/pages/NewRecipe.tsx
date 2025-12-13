import { useMutation } from "@tanstack/react-query";
import { useLocation } from "wouter";
import { RecipeEditor } from "@/components/RecipeEditor";
import { useToast } from "@/hooks/use-toast";
import { apiRequest, queryClient } from "@/lib/queryClient";

interface NewRecipeProps {
  onBack?: () => void;
}

export function NewRecipe({ onBack }: NewRecipeProps) {
  const [, setLocation] = useLocation();
  const { toast } = useToast();

  const createMutation = useMutation({
    mutationFn: async (data: { title: string; content: string; tags: string[]; isPublic: boolean }) => {
      const response = await apiRequest("POST", "/api/recipes", {
        title: data.title,
        content: data.content,
        isPublic: data.isPublic,
        chapters: data.tags,
      });
      return response.json();
    },
    onSuccess: (recipe) => {
      queryClient.invalidateQueries({ queryKey: ["/api/recipes"] });
      queryClient.invalidateQueries({ queryKey: ["/api/recipes/recent"] });
      queryClient.invalidateQueries({ queryKey: ["/api/stats"] });
      queryClient.invalidateQueries({ queryKey: ["/api/chapters"] });
      toast({
        title: "Recipe created",
        description: "Your recipe has been saved successfully.",
      });
      setLocation(`/recipe/${recipe.id}`);
    },
    onError: () => {
      toast({
        title: "Error",
        description: "Failed to create recipe. Please try again.",
        variant: "destructive",
      });
    },
  });

  return (
    <div className="max-w-3xl mx-auto">
      <RecipeEditor
        isNew
        onCancel={onBack}
        onSave={(data) => createMutation.mutate(data)}
      />
    </div>
  );
}
