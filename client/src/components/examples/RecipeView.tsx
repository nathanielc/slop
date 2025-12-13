import { RecipeView } from "../RecipeView";

export default function RecipeViewExample() {
  return (
    <RecipeView
      title="Classic Spaghetti Carbonara"
      authorName="Mario Rossi"
      authorInitials="MR"
      isPublic={true}
      tags={["Italian", "Pasta", "Quick", "Comfort Food"]}
      content={`## Ingredients
- 400g spaghetti
- 200g guanciale or pancetta
- 4 egg yolks
- 1 whole egg
- 100g Pecorino Romano
- Black pepper to taste

## Instructions
1. Bring a large pot of salted water to boil
2. Cook spaghetti until al dente
3. Cut guanciale into small cubes
4. Fry guanciale until crispy
5. Whisk eggs with grated cheese
6. Drain pasta, reserve some water
7. Toss hot pasta with guanciale
8. Remove from heat, add egg mixture
9. Toss quickly, add pasta water if needed
10. Season with black pepper
11. Serve immediately`}
      isOwner={true}
      onEdit={() => console.log("Edit clicked")}
      onAddToBook={() => console.log("Add to book clicked")}
    />
  );
}
