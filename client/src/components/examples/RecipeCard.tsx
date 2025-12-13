import { RecipeCard } from "../RecipeCard";

export default function RecipeCardExample() {
  return (
    <RecipeCard
      id="1"
      title="Classic Chocolate Chip Cookies"
      authorName="Jane Smith"
      authorInitials="JS"
      isPublic={true}
      tags={["Desserts", "Baking", "Quick", "Family Favorite"]}
      preview="2 cups flour, 1 cup butter, 1 cup sugar, 2 eggs, 1 tsp vanilla, chocolate chips..."
      onClick={() => console.log("Recipe clicked")}
    />
  );
}
