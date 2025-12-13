import { RecipeEditor } from "@/components/RecipeEditor";
import { Button } from "@/components/ui/button";
import { ArrowLeft } from "lucide-react";

interface NewRecipeProps {
  onBack?: () => void;
  onSave?: (data: { title: string; content: string; tags: string[]; isPublic: boolean }) => void;
}

export function NewRecipe({ onBack, onSave }: NewRecipeProps) {
  return (
    <div className="space-y-4">
      <Button variant="ghost" onClick={onBack} data-testid="button-back">
        <ArrowLeft className="h-4 w-4 mr-2" />
        Back
      </Button>
      <RecipeEditor
        isNew
        onSave={(data) => {
          console.log("New recipe created:", data);
          onSave?.(data);
          onBack?.();
        }}
        onCancel={onBack}
      />
    </div>
  );
}
