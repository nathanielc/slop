import { useState } from "react";
import { RecipeCard, RecipeCardProps } from "@/components/RecipeCard";
import { SearchBar } from "@/components/SearchBar";
import { EmptyState } from "@/components/EmptyState";

interface PublicRecipesProps {
  onRecipeClick?: (id: string) => void;
}

export function PublicRecipes({ onRecipeClick }: PublicRecipesProps) {
  const [search, setSearch] = useState("");

  // todo: remove mock functionality
  const mockRecipes: RecipeCardProps[] = [
    {
      id: "p1",
      title: "Perfect Pancakes",
      authorName: "Chef Mario",
      authorInitials: "CM",
      isPublic: true,
      tags: ["Breakfast", "Quick", "Family"],
      preview: "2 cups flour, 2 eggs, 1.5 cups milk, butter, maple syrup...",
    },
    {
      id: "p2",
      title: "Creamy Mushroom Risotto",
      authorName: "Anna K.",
      authorInitials: "AK",
      isPublic: true,
      tags: ["Italian", "Vegetarian", "Comfort Food"],
      preview: "Arborio rice, mushrooms, white wine, parmesan, vegetable stock...",
    },
    {
      id: "p3",
      title: "Beef Tacos",
      authorName: "Carlos M.",
      authorInitials: "CM",
      isPublic: true,
      tags: ["Mexican", "Quick Meals"],
      preview: "Ground beef, taco seasoning, corn tortillas, salsa, cheese...",
    },
    {
      id: "p4",
      title: "Classic French Omelette",
      authorName: "Pierre D.",
      authorInitials: "PD",
      isPublic: true,
      tags: ["French", "Breakfast", "Quick"],
      preview: "3 eggs, butter, fresh herbs, salt, pepper, technique is key...",
    },
    {
      id: "p5",
      title: "Homemade Ramen",
      authorName: "Yuki T.",
      authorInitials: "YT",
      isPublic: true,
      tags: ["Japanese", "Soup", "Comfort Food"],
      preview: "Rich pork broth, ramen noodles, soft-boiled egg, chashu pork...",
    },
    {
      id: "p6",
      title: "Mediterranean Salad",
      authorName: "Elena P.",
      authorInitials: "EP",
      isPublic: true,
      tags: ["Healthy", "Vegetarian", "Quick"],
      preview: "Tomatoes, cucumber, olives, feta cheese, olive oil, oregano...",
    },
  ];

  const filteredRecipes = mockRecipes.filter((r) => {
    if (search && !r.title.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="text-4xl font-bold">Discover Recipes</h1>
        <p className="text-muted-foreground">Explore recipes shared by our community</p>
      </div>

      <SearchBar
        value={search}
        onChange={setSearch}
        onSearch={setSearch}
        placeholder="Search public recipes..."
      />

      {filteredRecipes.length === 0 ? (
        <EmptyState type={search ? "no-results" : "no-public"} />
      ) : (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {filteredRecipes.map((recipe) => (
            <RecipeCard
              key={recipe.id}
              {...recipe}
              onClick={() => onRecipeClick?.(recipe.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
