import { StatsCard } from "../StatsCard";
import { ChefHat } from "lucide-react";

export default function StatsCardExample() {
  return (
    <StatsCard
      title="Total Recipes"
      value={42}
      icon={ChefHat}
      description="5 added this week"
    />
  );
}
