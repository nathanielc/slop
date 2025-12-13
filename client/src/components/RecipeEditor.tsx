import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";
import { Save, Trash2, X, Plus, Globe, Lock } from "lucide-react";

export interface RecipeEditorProps {
  initialTitle?: string;
  initialContent?: string;
  initialTags?: string[];
  initialIsPublic?: boolean;
  onSave?: (data: { title: string; content: string; tags: string[]; isPublic: boolean }) => void;
  onDelete?: () => void;
  onCancel?: () => void;
  isNew?: boolean;
}

export function RecipeEditor({
  initialTitle = "",
  initialContent = "",
  initialTags = [],
  initialIsPublic = false,
  onSave,
  onDelete,
  onCancel,
  isNew = false,
}: RecipeEditorProps) {
  const [title, setTitle] = useState(initialTitle);
  const [content, setContent] = useState(initialContent);
  const [tags, setTags] = useState<string[]>(initialTags);
  const [isPublic, setIsPublic] = useState(initialIsPublic);
  const [newTag, setNewTag] = useState("");

  const handleAddTag = () => {
    const trimmed = newTag.trim();
    if (trimmed && !tags.includes(trimmed)) {
      setTags([...tags, trimmed]);
      setNewTag("");
    }
  };

  const handleRemoveTag = (tagToRemove: string) => {
    setTags(tags.filter((t) => t !== tagToRemove));
  };

  const handleSave = () => {
    onSave?.({ title, content, tags, isPublic });
  };

  return (
    <Card className="w-full">
      <CardHeader className="flex flex-row items-center justify-between gap-4">
        <CardTitle className="font-serif text-2xl">
          {isNew ? "New Recipe" : "Edit Recipe"}
        </CardTitle>
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-2">
            {isPublic ? (
              <Globe className="h-4 w-4 text-muted-foreground" />
            ) : (
              <Lock className="h-4 w-4 text-muted-foreground" />
            )}
            <Label htmlFor="public-toggle" className="text-sm">
              {isPublic ? "Public" : "Private"}
            </Label>
            <Switch
              id="public-toggle"
              checked={isPublic}
              onCheckedChange={setIsPublic}
              data-testid="switch-public"
            />
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-2">
          <Label htmlFor="recipe-title">Title</Label>
          <Input
            id="recipe-title"
            placeholder="Recipe title..."
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="font-serif text-lg"
            data-testid="input-recipe-title"
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="recipe-content">Recipe</Label>
          <Textarea
            id="recipe-content"
            placeholder="Enter your recipe here...

## Ingredients
- 2 cups flour
- 1 cup sugar

## Instructions
1. Preheat oven to 350F
2. Mix dry ingredients..."
            value={content}
            onChange={(e) => setContent(e.target.value)}
            className="min-h-[300px] font-mono text-sm leading-relaxed resize-y"
            data-testid="textarea-recipe-content"
          />
        </div>

        <div className="space-y-2">
          <Label>Tags</Label>
          <div className="flex flex-wrap gap-2 mb-2">
            {tags.map((tag) => (
              <Badge key={tag} variant="secondary" className="gap-1">
                {tag}
                <button
                  onClick={() => handleRemoveTag(tag)}
                  className="ml-1 rounded-full hover-elevate"
                  data-testid={`button-remove-tag-${tag}`}
                >
                  <X className="h-3 w-3" />
                </button>
              </Badge>
            ))}
          </div>
          <div className="flex gap-2">
            <Input
              placeholder="Add a tag..."
              value={newTag}
              onChange={(e) => setNewTag(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && (e.preventDefault(), handleAddTag())}
              className="max-w-[200px]"
              data-testid="input-new-tag"
            />
            <Button
              type="button"
              variant="outline"
              size="icon"
              onClick={handleAddTag}
              data-testid="button-add-tag"
            >
              <Plus className="h-4 w-4" />
            </Button>
          </div>
        </div>

        <div className="flex items-center justify-between gap-4 pt-4 border-t">
          <div className="flex gap-2">
            {onCancel && (
              <Button variant="ghost" onClick={onCancel} data-testid="button-cancel">
                Cancel
              </Button>
            )}
            {!isNew && onDelete && (
              <Button variant="destructive" onClick={onDelete} data-testid="button-delete">
                <Trash2 className="h-4 w-4 mr-2" />
                Delete
              </Button>
            )}
          </div>
          <Button onClick={handleSave} data-testid="button-save">
            <Save className="h-4 w-4 mr-2" />
            Save Recipe
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
