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
  initialContent?: string;
  initialTags?: string[];
  initialIsPublic?: boolean;
  onSave?: (data: { title: string; content: string; tags: string[]; isPublic: boolean }) => void;
  onDelete?: () => void;
  onCancel?: () => void;
  isNew?: boolean;
}

function parseTitle(content: string): string {
  const lines = content.split('\n');
  const titleMatch = lines[0]?.match(/^#\s*(.+)/) || lines[0]?.match(/^(.+)/);
  return titleMatch?.[1]?.trim() || "Untitled Recipe";
}

export function RecipeEditor({
  initialContent = "",
  initialTags = [],
  initialIsPublic = false,
  onSave,
  onDelete,
  onCancel,
  isNew = false,
}: RecipeEditorProps) {
  const [content, setContent] = useState(initialContent);
  const [tags, setTags] = useState<string[]>(initialTags);
  const [isPublic, setIsPublic] = useState(initialIsPublic);
  const [newTag, setNewTag] = useState("");

  const title = parseTitle(content);

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
        <div className="space-y-1">
          <p className="text-sm text-muted-foreground">{isNew ? "New Recipe" : "Edit Recipe"}</p>
          <CardTitle className="font-serif text-2xl" data-testid="text-parsed-title">
            {title}
          </CardTitle>
        </div>
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
          <Label htmlFor="recipe-content">Recipe (first line becomes the title)</Label>
          <Textarea
            id="recipe-content"
            placeholder="<** Souffle pancake with one egg
*3 or 4 drops: lemon juice
*1 : egg =separate keep white #stir in =beat at medium speed, until foamy
*1 1/2 tbsp: sugar #sprinkle in =beat at medium speed 3m until firm peaks form
*^egg yolk
*2 tbsp: flour #+
*1 tbsp: milk #mix to combine
*1/2 tsp: vanilla #stir in
*^1/3 of: egg white mixture #mix with circular motion
    #fold in with flat spatula
*1 tsp: oil =heat in pan 1m
*^2/3 of: pancake mixture #scoop into pan as two pancakes
*2 tsp: water #add to sides of pan =cover cook 2m on medium heat
*1 tsp: water #add to sides of pan #place on top
    =cover cook 5m on medium low heat
    =flip
    =cover cook 5m
    =serve with fruit and syrup/powdered sugar
#* Makes 2 pancakes
>"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            className="min-h-[300px] font-mono text-sm leading-relaxed resize-y"
            data-testid="textarea-recipe-content"
          />
        </div>

        <div className="space-y-2">
          <Label>Chapters</Label>
          <div className="flex flex-wrap gap-2 mb-2">
            {tags.map((tag) => (
              <Badge key={tag} variant="secondary" className="gap-1">
                {tag}
                <button
                  onClick={() => handleRemoveTag(tag)}
                  className="ml-1 rounded-full hover-elevate"
                  data-testid={`button-remove-chapter-${tag}`}
                >
                  <X className="h-3 w-3" />
                </button>
              </Badge>
            ))}
          </div>
          <div className="flex gap-2">
            <Input
              placeholder="Add to chapter..."
              value={newTag}
              onChange={(e) => setNewTag(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && (e.preventDefault(), handleAddTag())}
              className="max-w-[200px]"
              data-testid="input-new-chapter"
            />
            <Button
              type="button"
              variant="outline"
              size="icon"
              onClick={handleAddTag}
              data-testid="button-add-chapter"
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
