import { RecipeEditor } from "../RecipeEditor";

export default function RecipeEditorExample() {
  return (
    <RecipeEditor
      initialTitle="Grandma's Apple Pie"
      initialContent={`## Ingredients
- 6 cups sliced apples
- 3/4 cup sugar
- 2 tbsp flour
- 1 tsp cinnamon
- 1/4 tsp nutmeg
- 1 tbsp butter
- Pastry for double-crust pie

## Instructions
1. Preheat oven to 425F
2. Mix sugar, flour, cinnamon, and nutmeg
3. Toss apples with mixture
4. Fill pie crust, dot with butter
5. Cover with top crust
6. Bake 40-50 minutes`}
      initialTags={["Desserts", "Baking", "Family"]}
      initialIsPublic={true}
      onSave={(data) => console.log("Save:", data)}
      onDelete={() => console.log("Delete")}
      onCancel={() => console.log("Cancel")}
    />
  );
}
