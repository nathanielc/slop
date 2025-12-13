import { useState } from "react";
import { TagSelector } from "../TagSelector";

export default function TagSelectorExample() {
  const [availableTags, setAvailableTags] = useState([
    "Breakfast",
    "Lunch", 
    "Dinner",
    "Desserts",
    "Vegetarian",
    "Quick Meals",
  ]);
  const [selectedTags, setSelectedTags] = useState(["Dinner", "Quick Meals"]);

  return (
    <TagSelector
      availableTags={availableTags}
      selectedTags={selectedTags}
      onTagsChange={setSelectedTags}
      onCreateTag={(tag) => setAvailableTags([...availableTags, tag])}
    />
  );
}
