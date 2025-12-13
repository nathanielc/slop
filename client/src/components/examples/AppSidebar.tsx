import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "../AppSidebar";

export default function AppSidebarExample() {
  return (
    <SidebarProvider>
      <AppSidebar
        username="Jane Smith"
        initials="JS"
        tags={[
          { name: "Desserts", count: 12 },
          { name: "Quick Meals", count: 8 },
          { name: "Italian", count: 5 },
          { name: "Healthy", count: 3 },
        ]}
        onNewRecipe={() => console.log("New recipe")}
      />
    </SidebarProvider>
  );
}
