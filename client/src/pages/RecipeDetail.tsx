import { useState } from "react";
import { RecipeView } from "@/components/RecipeView";
import { RecipeEditor } from "@/components/RecipeEditor";
import { Button } from "@/components/ui/button";
import { ArrowLeft } from "lucide-react";

interface RecipeDetailProps {
  recipeId: string;
  username: string;
  onBack?: () => void;
}

export function RecipeDetail({ recipeId, username, onBack }: RecipeDetailProps) {
  const [isEditing, setIsEditing] = useState(false);
  const initials = username.split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();

  // todo: remove mock functionality
  const mockRecipe = {
    id: recipeId,
    title: "Classic Spaghetti Carbonara",
    authorName: username,
    authorInitials: initials,
    isPublic: true,
    tags: ["Italian", "Pasta", "Quick Meals", "Comfort Food"],
    content: `## Ingredients
- 400g spaghetti
- 200g guanciale or pancetta
- 4 egg yolks
- 1 whole egg
- 100g Pecorino Romano, finely grated
- Freshly ground black pepper

## Instructions
1. Bring a large pot of salted water to boil
2. Cook spaghetti according to package directions until al dente
3. While pasta cooks, cut guanciale into small cubes
4. Fry guanciale in a large pan until crispy and golden
5. In a bowl, whisk together egg yolks, whole egg, and grated cheese
6. Drain pasta, reserving 1 cup of pasta water
7. Add hot pasta to the pan with guanciale
8. Remove from heat and quickly add egg mixture
9. Toss vigorously, adding pasta water as needed for creaminess
10. Season generously with black pepper
11. Serve immediately with extra cheese

## Tips
- Never add the eggs while the pan is on heat - they will scramble
- The residual heat from the pasta cooks the eggs into a silky sauce
- Authentic carbonara never uses cream - the creaminess comes from the eggs and cheese`,
  };

  const [recipe, setRecipe] = useState(mockRecipe);

  if (isEditing) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={() => setIsEditing(false)} data-testid="button-back-to-view">
          <ArrowLeft className="h-4 w-4 mr-2" />
          Back to recipe
        </Button>
        <RecipeEditor
          initialTitle={recipe.title}
          initialContent={recipe.content}
          initialTags={recipe.tags}
          initialIsPublic={recipe.isPublic}
          onSave={(data) => {
            setRecipe({ ...recipe, ...data });
            setIsEditing(false);
            console.log("Recipe saved:", data);
          }}
          onCancel={() => setIsEditing(false)}
          onDelete={() => {
            console.log("Recipe deleted");
            onBack?.();
          }}
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
        {...recipe}
        isOwner={true}
        onEdit={() => setIsEditing(true)}
        onAddToBook={() => console.log("Add to book clicked")}
      />
    </div>
  );
}
