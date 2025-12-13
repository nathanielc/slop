import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Filter } from "lucide-react";

export type VisibilityFilter = "all" | "public" | "private";
export type SortOption = "newest" | "oldest" | "title";

export interface RecipeFilterProps {
  visibility: VisibilityFilter;
  sort: SortOption;
  onVisibilityChange: (value: VisibilityFilter) => void;
  onSortChange: (value: SortOption) => void;
}

export function RecipeFilter({
  visibility,
  sort,
  onVisibilityChange,
  onSortChange,
}: RecipeFilterProps) {
  return (
    <div className="flex flex-wrap items-center gap-2">
      <div className="flex items-center gap-1 text-sm text-muted-foreground">
        <Filter className="h-4 w-4" />
        <span className="hidden sm:inline">Filter:</span>
      </div>
      <Select value={visibility} onValueChange={(v) => onVisibilityChange(v as VisibilityFilter)}>
        <SelectTrigger className="w-[120px]" data-testid="select-visibility">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All</SelectItem>
          <SelectItem value="public">Public</SelectItem>
          <SelectItem value="private">Private</SelectItem>
        </SelectContent>
      </Select>
      <Select value={sort} onValueChange={(v) => onSortChange(v as SortOption)}>
        <SelectTrigger className="w-[130px]" data-testid="select-sort">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="newest">Newest</SelectItem>
          <SelectItem value="oldest">Oldest</SelectItem>
          <SelectItem value="title">Title A-Z</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
}
