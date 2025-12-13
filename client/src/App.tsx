import { useState } from "react";
import { Switch, Route, useLocation } from "wouter";
import { queryClient } from "./lib/queryClient";
import { QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "@/components/ui/toaster";
import { TooltipProvider } from "@/components/ui/tooltip";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/AppSidebar";
import { ThemeToggle } from "@/components/ThemeToggle";
import { UserMenu } from "@/components/UserMenu";
import { LoginCard } from "@/components/LoginCard";
import { Dashboard } from "@/pages/Dashboard";
import { MyRecipes } from "@/pages/MyRecipes";
import { PublicRecipes } from "@/pages/PublicRecipes";
import { RecipeBook } from "@/pages/RecipeBook";
import { RecipeDetail } from "@/pages/RecipeDetail";
import { NewRecipe } from "@/pages/NewRecipe";
import NotFound from "@/pages/not-found";

function AppContent() {
  const [, setLocation] = useLocation();

  // todo: remove mock functionality
  const [isLoggedIn, setIsLoggedIn] = useState(true);
  const mockUser = {
    username: "Jane Smith",
    initials: "JS",
  };

  const mockTags = [
    { name: "Desserts", count: 12 },
    { name: "Quick Meals", count: 8 },
    { name: "Italian", count: 5 },
    { name: "Healthy", count: 3 },
  ];

  if (!isLoggedIn) {
    return <LoginCard onLogin={() => setIsLoggedIn(true)} />;
  }

  const sidebarStyle = {
    "--sidebar-width": "16rem",
    "--sidebar-width-icon": "3rem",
  };

  return (
    <SidebarProvider style={sidebarStyle as React.CSSProperties}>
      <div className="flex h-screen w-full">
        <AppSidebar
          username={mockUser.username}
          initials={mockUser.initials}
          tags={mockTags}
          onNewRecipe={() => setLocation("/new")}
        />
        <div className="flex flex-col flex-1 min-w-0">
          <header className="flex items-center justify-between gap-4 p-4 border-b sticky top-0 z-50 bg-background">
            <SidebarTrigger data-testid="button-sidebar-toggle" />
            <div className="flex items-center gap-2">
              <ThemeToggle />
              <UserMenu
                username={mockUser.username}
                initials={mockUser.initials}
                onProfile={() => console.log("Profile")}
                onSettings={() => setLocation("/settings")}
                onLogout={() => setIsLoggedIn(false)}
              />
            </div>
          </header>
          <main className="flex-1 overflow-auto p-6">
            <Switch>
              <Route path="/">
                <Dashboard
                  username={mockUser.username}
                  onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                />
              </Route>
              <Route path="/recipes">
                <MyRecipes
                  username={mockUser.username}
                  onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                  onNewRecipe={() => setLocation("/new")}
                />
              </Route>
              <Route path="/public">
                <PublicRecipes
                  onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                />
              </Route>
              <Route path="/book/:tagName">
                {(params) => (
                  <RecipeBook
                    tagName={decodeURIComponent(params.tagName)}
                    username={mockUser.username}
                    onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                    onEditTag={() => console.log("Edit tag")}
                    onDeleteTag={() => console.log("Delete tag")}
                    onBrowseRecipes={() => setLocation("/recipes")}
                  />
                )}
              </Route>
              <Route path="/recipe/:id">
                {(params) => (
                  <RecipeDetail
                    recipeId={params.id}
                    username={mockUser.username}
                    onBack={() => window.history.back()}
                  />
                )}
              </Route>
              <Route path="/new">
                <NewRecipe
                  onBack={() => window.history.back()}
                  onSave={(data) => console.log("Recipe saved:", data)}
                />
              </Route>
              <Route component={NotFound} />
            </Switch>
          </main>
        </div>
      </div>
    </SidebarProvider>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <Toaster />
        <AppContent />
      </TooltipProvider>
    </QueryClientProvider>
  );
}

export default App;
