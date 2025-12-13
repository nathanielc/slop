import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Check, Plus, Tag } from "lucide-react";

export interface TagSelectorProps {
  availableTags: string[];
  selectedTags: string[];
  onTagsChange: (tags: string[]) => void;
  onCreateTag?: (tag: string) => void;
}

export function TagSelector({
  availableTags,
  selectedTags,
  onTagsChange,
  onCreateTag,
}: TagSelectorProps) {
  const [open, setOpen] = useState(false);
  const [newTagValue, setNewTagValue] = useState("");

  const toggleTag = (tag: string) => {
    if (selectedTags.includes(tag)) {
      onTagsChange(selectedTags.filter((t) => t !== tag));
    } else {
      onTagsChange([...selectedTags, tag]);
    }
  };

  const handleCreateTag = () => {
    const trimmed = newTagValue.trim();
    if (trimmed && !availableTags.includes(trimmed)) {
      onCreateTag?.(trimmed);
      onTagsChange([...selectedTags, trimmed]);
      setNewTagValue("");
    }
  };

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="outline" className="gap-2" data-testid="button-tag-selector">
          <Tag className="h-4 w-4" />
          {selectedTags.length > 0 ? `${selectedTags.length} tags` : "Select tags"}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-64 p-2" align="start">
        <div className="space-y-2">
          <div className="max-h-[200px] overflow-auto space-y-1">
            {availableTags.map((tag) => {
              const isSelected = selectedTags.includes(tag);
              return (
                <button
                  key={tag}
                  onClick={() => toggleTag(tag)}
                  className="w-full flex items-center justify-between px-2 py-1.5 text-sm rounded-md hover-elevate active-elevate-2"
                  data-testid={`tag-option-${tag}`}
                >
                  <span>{tag}</span>
                  {isSelected && <Check className="h-4 w-4 text-primary" />}
                </button>
              );
            })}
          </div>
          {onCreateTag && (
            <>
              <div className="border-t pt-2">
                <div className="flex gap-1">
                  <Input
                    placeholder="New tag..."
                    value={newTagValue}
                    onChange={(e) => setNewTagValue(e.target.value)}
                    onKeyDown={(e) => e.key === "Enter" && handleCreateTag()}
                    className="h-8 text-sm"
                    data-testid="input-create-tag"
                  />
                  <Button
                    size="icon"
                    variant="ghost"
                    onClick={handleCreateTag}
                    className="h-8 w-8 shrink-0"
                    data-testid="button-create-tag"
                  >
                    <Plus className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            </>
          )}
        </div>
      </PopoverContent>
    </Popover>
  );
}
