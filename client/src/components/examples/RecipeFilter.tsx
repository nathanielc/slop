import { useState } from "react";
import { RecipeFilter, VisibilityFilter, SortOption } from "../RecipeFilter";

export default function RecipeFilterExample() {
  const [visibility, setVisibility] = useState<VisibilityFilter>("all");
  const [sort, setSort] = useState<SortOption>("newest");

  return (
    <RecipeFilter
      visibility={visibility}
      sort={sort}
      onVisibilityChange={setVisibility}
      onSortChange={setSort}
    />
  );
}
